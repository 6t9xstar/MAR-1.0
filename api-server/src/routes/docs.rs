use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models;
use crate::SharedState;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::auth::register,
        crate::routes::auth::login,
        crate::routes::auth::refresh_token,
        crate::routes::auth::get_profile,
        crate::routes::auth::update_profile,
        crate::routes::chat::list_conversations,
        crate::routes::chat::create_conversation,
        crate::routes::chat::send_message,
        crate::routes::chat::stream_message,
    ),
    components(
        schemas(
            models::user::RegisterRequest,
            models::user::LoginRequest,
            models::user::AuthResponse,
            models::user::UserPublic,
            models::user::TokenRefreshRequest,
            models::user::TokenRefreshResponse,
            models::user::UpdateProfileRequest,
            models::user::UserProfile,
            models::chat::Conversation,
            models::chat::ConversationSummary,
            models::chat::CreateConversationRequest,
            models::chat::UpdateConversationRequest,
            models::chat::ChatMessage,
            models::chat::SendMessageRequest,
            models::chat::MessageResponse,
            models::chat::StreamChunk,
            models::memory::MemoryEntry,
            models::memory::CreateMemoryRequest,
            models::memory::UpdateMemoryRequest,
            models::memory::MemorySearchResult,
            models::memory::MemoryStats,
            models::document::Document,
            models::document::DocumentSummary,
            models::document::DocumentQueryResult,
            models::tool::ToolDefinition,
            models::tool::ToolCall,
            models::tool::ToolResult,
            models::HealthResponse,
            models::PaginationParams,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication & user management"),
        (name = "Chat", description = "Conversations & messaging"),
        (name = "Memory", description = "User memory management"),
        (name = "Documents", description = "File upload & document Q&A"),
        (name = "Tools", description = "Built-in tools"),
        (name = "Admin", description = "Health & administration"),
    ),
    info(
        title = "MAR 1.0 API",
        version = "1.0.0",
        description = "MAR 1.0 - AI assistant for Pakistan",
    )
)]
pub struct ApiDoc;

pub fn openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

pub fn router(openapi: utoipa::openapi::OpenApi) -> Router<SharedState> {
    let swagger = SwaggerUi::new("/api/docs").url("/api/docs/openapi.json", openapi);
    Router::new().merge(swagger)
}
