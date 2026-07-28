use crate::errors::{AppError, AppResult};
use crate::models::tool::{BuiltInTool, ToolCall, ToolDefinition, ToolResult};
use chrono::Utc;
use serde_json::Value;
use std::time::Instant;

pub struct ToolService;

impl ToolService {
    pub fn available_tools() -> Vec<ToolDefinition> {
        vec![
            BuiltInTool::WebSearch.definition(),
            BuiltInTool::Calculator.definition(),
            BuiltInTool::Translation.definition(),
            BuiltInTool::Weather.definition(),
            BuiltInTool::CurrentTime.definition(),
        ]
    }

    pub async fn execute(tool_calls: Vec<ToolCall>) -> AppResult<Vec<ToolResult>> {
        let mut results = Vec::with_capacity(tool_calls.len());

        for call in tool_calls {
            let start = Instant::now();
            let result = match call.tool_name.as_str() {
                "web_search" => Self::web_search(&call.arguments).await,
                "calculator" => Self::calculator(&call.arguments),
                "translate" => Self::translate(&call.arguments).await,
                "get_weather" => Self::weather(&call.arguments).await,
                "current_time" => Self::current_time(&call.arguments),
                _ => Err(AppError::BadRequest(format!("Unknown tool: {}", call.tool_name))),
            };

            let duration_ms = start.elapsed().as_millis() as u64;

            match result {
                Ok(output) => results.push(ToolResult {
                    call_id: call.id,
                    success: true,
                    output,
                    error: None,
                    duration_ms,
                }),
                Err(e) => results.push(ToolResult {
                    call_id: call.id,
                    success: false,
                    output: Value::Null,
                    error: Some(e.to_string()),
                    duration_ms,
                }),
            }
        }

        Ok(results)
    }

    async fn web_search(args: &Value) -> AppResult<Value> {
        let query = args["query"].as_str()
            .ok_or_else(|| AppError::Validation("Missing 'query' parameter".into()))?;
        let _limit = args["limit"].as_u64().unwrap_or(5).min(20) as usize;

        // In production, integrate with a search API
        Ok(serde_json::json!({
            "query": query,
            "results": [],
            "total_results": 0,
            "note": "Web search integration pending - connect a search API key in production"
        }))
    }

    fn calculator(args: &Value) -> AppResult<Value> {
        let expression = args["expression"].as_str()
            .ok_or_else(|| AppError::Validation("Missing 'expression' parameter".into()))?;

        // Safe math evaluation using meval crate
        match meval::eval_str(expression) {
            Ok(result) => Ok(serde_json::json!({
                "expression": expression,
                "result": result,
                "formatted": if result.fract() == 0.0 {
                    format!("{}", result as i64)
                } else {
                    format!("{:.6}", result)
                }
            })),
            Err(e) => Err(AppError::Validation(format!("Invalid expression: {e}"))),
        }
    }

    async fn translate(args: &Value) -> AppResult<Value> {
        let text = args["text"].as_str()
            .ok_or_else(|| AppError::Validation("Missing 'text' parameter".into()))?;
        let target = args["target_language"].as_str()
            .ok_or_else(|| AppError::Validation("Missing 'target_language' parameter".into()))?;
        let source = args["source_language"].as_str().unwrap_or("auto");

        Ok(serde_json::json!({
            "source_text": text,
            "source_language": source,
            "target_language": target,
            "translated_text": "[Translation integration pending]",
            "note": "Connect a translation API key in production"
        }))
    }

    async fn weather(args: &Value) -> AppResult<Value> {
        let location = args["location"].as_str()
            .ok_or_else(|| AppError::Validation("Missing 'location' parameter".into()))?;

        Ok(serde_json::json!({
            "location": location,
            "temperature": null,
            "conditions": null,
            "humidity": null,
            "note": "Weather integration pending - connect a weather API key in production"
        }))
    }

    fn current_time(args: &Value) -> AppResult<Value> {
        let timezone = args["timezone"].as_str().unwrap_or("UTC");
        let now = Utc::now();

        Ok(serde_json::json!({
            "timezone": timezone,
            "utc_time": now.to_rfc3339(),
            "unix_timestamp": now.timestamp(),
            "formatted": now.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        }))
    }
}
