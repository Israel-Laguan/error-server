use serde_json::{json, Value};
use std::env;

use poem::{handler, web::Json, web::Path};

use crate::features::database::check_db_is_reachable;

#[handler]
pub fn about() -> Json<Value> {
    const ID: &str = env!("CARGO_PKG_NAME");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    Json(json!({
        "id": ID,
        "description": DESCRIPTION,
        "version": VERSION,
    }))
}

#[handler]
pub fn am_i_up() -> String {
    "OK".to_string()
}

#[handler]
pub async fn dependency(Path(dependency): Path<String>) -> Json<serde_json::Value> {
    match dependency.as_str() {
        "db" => {
            let db_status: bool = check_db_is_reachable().await;
            Json(json!({
                "ok": db_status.to_string(),
                "message": "Database state was check",
            }))
        },
        _ => Json(json!({
            "ok": "false",
            "message": "Please provide a dependency, or just use /about"
        })),
    }
}
