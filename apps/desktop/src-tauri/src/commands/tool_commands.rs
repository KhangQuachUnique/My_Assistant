use serde_json::Value;

use crate::services::tool_service;

#[tauri::command]
pub async fn run_tool(
    tool_name: String,
    args: Value,
) -> Result<Value, String> {
    tool_service::run_tool(tool_name, args)
        .await
        .map_err(|err| err.to_string())
}