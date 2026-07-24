use serde_json::{json, Value};

use crate::services::app_service;

pub async fn run_tool(tool_name: String, args: Value) -> Result<Value, String> {
    match tool_name.as_str() {
        "open_app" => open_app_tool(args).await,
        _ => Err(format!("Tool does not exist: {}", tool_name)),
    }
}

async fn open_app_tool(args: Value) -> Result<Value, String> {
    let app = args
        .get("app")
        .and_then(|value| value.as_str())
        .ok_or("Missing args.app")?;

    let resolved_target = app_service::open_app(app)?;

    Ok(json!({
        "success": true,
        "message": format!("Opened app: {} ({})", app, resolved_target),
        "data": {
            "app": app,
            "resolvedTarget": resolved_target
        }
    }))
}
