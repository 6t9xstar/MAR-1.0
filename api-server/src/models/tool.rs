use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ToolCall {
    pub id: String,
    pub tool_name: String,
    pub arguments: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ToolResult {
    pub call_id: String,
    pub success: bool,
    pub output: Value,
    pub error: Option<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ToolExecutionRequest {
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ToolExecutionResponse {
    pub results: Vec<ToolResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum BuiltInTool {
    WebSearch,
    Calculator,
    Translation,
    Weather,
    PdfReader,
    CodeExecution,
    CurrentTime,
    KnowledgeRetrieval,
}

impl BuiltInTool {
    pub fn definition(&self) -> ToolDefinition {
        match self {
            Self::WebSearch => ToolDefinition {
                name: "web_search".into(),
                description: "Search the web for current information".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Search query"},
                        "limit": {"type": "integer", "description": "Max results", "default": 5}
                    },
                    "required": ["query"]
                }),
                required: vec!["query".into()],
            },
            Self::Calculator => ToolDefinition {
                name: "calculator".into(),
                description: "Perform mathematical calculations".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "expression": {"type": "string", "description": "Math expression to evaluate"}
                    },
                    "required": ["expression"]
                }),
                required: vec!["expression".into()],
            },
            Self::Translation => ToolDefinition {
                name: "translate".into(),
                description: "Translate text between languages".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "text": {"type": "string", "description": "Text to translate"},
                        "source_language": {"type": "string", "description": "Source language"},
                        "target_language": {"type": "string", "description": "Target language"}
                    },
                    "required": ["text", "target_language"]
                }),
                required: vec!["text".into(), "target_language".into()],
            },
            Self::Weather => ToolDefinition {
                name: "get_weather".into(),
                description: "Get current weather for a location".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "location": {"type": "string", "description": "City or location name"}
                    },
                    "required": ["location"]
                }),
                required: vec!["location".into()],
            },
            Self::CurrentTime => ToolDefinition {
                name: "current_time".into(),
                description: "Get the current date and time".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "timezone": {"type": "string", "description": "Timezone (e.g. Asia/Karachi)", "default": "UTC"}
                    }
                }),
                required: vec![],
            },
            _ => ToolDefinition {
                name: "tool".into(),
                description: "A tool".into(),
                parameters: serde_json::json!({}),
                required: vec![],
            },
        }
    }
}
