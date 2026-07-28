use axum::routing::{get, post, put};
use axum::{Json, Router};

use crate::errors::AppResult;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::{
    AuthResponse, LoginRequest, RegisterRequest, TokenRefreshRequest,
    TokenRefreshResponse, UpdateProfileRequest, UserProfile,
};
use crate::services::auth::AuthService;
use crate::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/refresh", post(refresh_token))
        .route("/api/auth/me", get(get_profile))
        .route("/api/auth/me", put(update_profile))
        .route("/api/auth/me/memory", put(toggle_memory))
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, body = AuthResponse),
        (status = 409, description = "Username or email already exists"),
    ),
    tag = "Auth"
)]
async fn register(
    state: axum::extract::State<SharedState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<(axum::http::StatusCode, Json<AuthResponse>)> {
    let response = AuthService::register(&state.db, &state.config, req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
    ),
    tag = "Auth"
)]
async fn login(
    state: axum::extract::State<SharedState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let response = AuthService::login(&state.db, &state.config, req).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    request_body = TokenRefreshRequest,
    responses(
        (status = 200, body = TokenRefreshResponse),
        (status = 401, description = "Invalid refresh token"),
    ),
    tag = "Auth"
)]
async fn refresh_token(
    state: axum::extract::State<SharedState>,
    Json(req): Json<TokenRefreshRequest>,
) -> AppResult<Json<TokenRefreshResponse>> {
    let response = AuthService::refresh_token(&state.db, &state.config, &req.refresh_token).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, body = UserProfile),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Auth"
)]
async fn get_profile(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
) -> AppResult<Json<UserProfile>> {
    let profile = AuthService::get_profile(&state.db, user.id).await?;
    Ok(Json(profile))
}

#[utoipa::path(
    put,
    path = "/api/auth/me",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, body = UserProfile),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Auth"
)]
async fn update_profile(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Json(req): Json<UpdateProfileRequest>,
) -> AppResult<Json<UserProfile>> {
    let profile = AuthService::update_profile(&state.db, user.id, req).await?;
    Ok(Json(profile))
}

#[utoipa::path(
    put,
    path = "/api/auth/me/memory",
    responses(
        (status = 200, description = "Memory toggled"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Auth"
)]
async fn toggle_memory(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    let enabled = body["enabled"].as_bool().unwrap_or(true);
    AuthService::toggle_memory(&state.db, user.id, enabled).await?;
    Ok(Json(serde_json::json!({ "memory_enabled": enabled })))
}
