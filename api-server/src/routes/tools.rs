use axum::routing::{get, post};
use axum::{Json, Router};

use crate::errors::AppResult;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::tool::{ToolDefinition, ToolExecutionRequest, ToolResult};
use crate::services::tools::ToolService;
use crate::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/tools", get(list_tools))
        .route("/api/tools/execute", post(execute_tools))
}

async fn list_tools() -> Json<Vec<ToolDefinition>> {
    Json(ToolService::available_tools())
}

async fn execute_tools(
    _user: AuthenticatedUser,
    Json(req): Json<ToolExecutionRequest>,
) -> AppResult<Json<Vec<ToolResult>>> {
    let results = ToolService::execute(req.tool_calls).await?;
    Ok(Json(results))
}
