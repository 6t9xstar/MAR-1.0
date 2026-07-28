use crate::config::Config;
use crate::errors::{AppError, AppResult};
use crate::middleware::auth::{create_token, validate_token};
use crate::models::user::{
    AuthResponse, JwtClaims, LoginRequest, RegisterRequest,
    TokenRefreshResponse, UpdateProfileRequest, UserProfile, UserPublic,
};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use argon2::password_hash::rand_core::OsRng;
use sqlx::PgPool;
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub async fn register(db: &PgPool, config: &Config, req: RegisterRequest) -> AppResult<AuthResponse> {
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE username = $1 OR email = $2"
        )
        .bind(&req.username)
        .bind(&req.email)
        .fetch_one(db)
        .await?;

        if existing > 0 {
            return Err(AppError::Conflict("Username or email already exists".into()));
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))?
            .to_string();

        let id = Uuid::now_v7();
        let now = Utc::now();
        let display_name = req.display_name.unwrap_or_else(|| req.username.clone());
        let language = req.preferred_language.unwrap_or_else(|| "en".into());

        sqlx::query(
            r#"INSERT INTO users (id, username, email, password_hash, display_name, preferred_language, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#
        )
        .bind(id)
        .bind(&req.username)
        .bind(&req.email)
        .bind(&password_hash)
        .bind(&display_name)
        .bind(&language)
        .bind(now)
        .bind(now)
        .execute(db)
        .await?;

        Self::generate_auth_response(db, config, id, &req.username).await
    }

    pub async fn login(db: &PgPool, config: &Config, req: LoginRequest) -> AppResult<AuthResponse> {
        let user = sqlx::query_as::<_, (Uuid, String, String, String, Option<String>, String, bool, bool)>(
            r#"SELECT id, username, email, password_hash, display_name, preferred_language, is_verified, memory_enabled
               FROM users WHERE username = $1 OR email = $1"#
        )
        .bind(&req.username_or_email)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

        let parsed_hash = PasswordHash::new(&user.3)
            .map_err(|e| AppError::Internal(format!("Invalid password hash: {e}")))?;

        if Argon2::default().verify_password(req.password.as_bytes(), &parsed_hash).is_err() {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }

        Self::generate_auth_response(db, config, user.0, &user.1).await
    }

    pub async fn refresh_token(
        db: &PgPool,
        config: &Config,
        refresh_token: &str,
    ) -> AppResult<TokenRefreshResponse> {
        let claims = validate_token(refresh_token, &config.auth.jwt_secret)?;

        let user = sqlx::query_as::<_, (Uuid, String)>(
            "SELECT id, username FROM users WHERE id = $1"
        )
        .bind(claims.sub)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("User not found".into()))?;

        let now = Utc::now();
        let exp = now.timestamp() as usize + config.auth.jwt_expiration_secs as usize;

        let new_claims = JwtClaims {
            sub: user.0,
            username: user.1,
            exp,
            iat: now.timestamp() as usize,
            jti: Uuid::now_v7(),
        };

        let token = create_token(&new_claims, &config.auth.jwt_secret)?;

        Ok(TokenRefreshResponse {
            token,
            expires_at: chrono::DateTime::from_timestamp(exp as i64, 0).unwrap_or(now),
        })
    }

    pub async fn get_profile(db: &PgPool, user_id: Uuid) -> AppResult<UserProfile> {
        let user = sqlx::query_as::<_, (Uuid, String, String, Option<String>, String, bool, bool, DateTime<Utc>, DateTime<Utc>)>(
            r#"SELECT id, username, email, display_name, preferred_language, is_verified, memory_enabled, created_at, updated_at
               FROM users WHERE id = $1"#
        )
        .bind(user_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let conv_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM conversations WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(db)
        .await?;

        let msg_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE conversation_id IN (SELECT id FROM conversations WHERE user_id = $1)"
        )
        .bind(user_id)
        .fetch_one(db)
        .await?;

        Ok(UserProfile {
            id: user.0,
            username: user.1,
            email: user.2,
            display_name: user.3,
            preferred_language: user.4,
            is_verified: user.5,
            memory_enabled: user.6,
            total_conversations: conv_count.0 as u64,
            total_messages: msg_count.0 as u64,
            created_at: user.7,
            updated_at: user.8,
        })
    }

    pub async fn update_profile(
        db: &PgPool,
        user_id: Uuid,
        req: UpdateProfileRequest,
    ) -> AppResult<UserProfile> {
        if let Some(name) = &req.display_name {
            sqlx::query("UPDATE users SET display_name = $1, updated_at = $2 WHERE id = $3")
                .bind(name)
                .bind(Utc::now())
                .bind(user_id)
                .execute(db)
                .await?;
        }

        if let Some(lang) = &req.preferred_language {
            sqlx::query("UPDATE users SET preferred_language = $1, updated_at = $2 WHERE id = $3")
                .bind(lang)
                .bind(Utc::now())
                .bind(user_id)
                .execute(db)
                .await?;
        }

        Self::get_profile(db, user_id).await
    }

    pub async fn toggle_memory(db: &PgPool, user_id: Uuid, enabled: bool) -> AppResult<()> {
        sqlx::query("UPDATE users SET memory_enabled = $1 WHERE id = $2")
            .bind(enabled)
            .bind(user_id)
            .execute(db)
            .await?;
        Ok(())
    }

    async fn generate_auth_response(
        db: &PgPool,
        config: &Config,
        user_id: Uuid,
        username: &str,
    ) -> AppResult<AuthResponse> {
        let now = Utc::now();
        let exp = now.timestamp() as usize + config.auth.jwt_expiration_secs as usize;
        let refresh_exp = now.timestamp() as usize + config.auth.refresh_token_expiration_secs as usize;

        let claims = JwtClaims {
            sub: user_id,
            username: username.to_string(),
            exp,
            iat: now.timestamp() as usize,
            jti: Uuid::now_v7(),
        };

        let refresh_claims = JwtClaims {
            sub: user_id,
            username: String::new(),
            exp: refresh_exp,
            iat: now.timestamp() as usize,
            jti: Uuid::now_v7(),
        };

        let token = create_token(&claims, &config.auth.jwt_secret)?;
        let refresh_token = create_token(&refresh_claims, &config.auth.jwt_secret)?;

        let user = Self::get_profile(db, user_id).await?;

        Ok(AuthResponse {
            token,
            refresh_token,
            expires_at: chrono::DateTime::from_timestamp(exp as i64, 0).unwrap_or(now),
            user: UserPublic {
                id: user.id,
                username: user.username,
                email: user.email,
                display_name: user.display_name,
                preferred_language: user.preferred_language,
                is_verified: user.is_verified,
                created_at: user.created_at,
                memory_enabled: user.memory_enabled,
            },
        })
    }
}
