//! Axum hello-world for OctoPilot samples.
//! GET /greet?name=...&birth_year=... returns greeting and age confirmation.

use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::{Datelike, Utc};
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
struct GreetQuery {
    name: String,
    birth_year: i32,
}

#[derive(Debug, Serialize)]
struct GreetResponse {
    greeting: String,
    age_confirmation: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/greet", get(greet))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Hello World! Use GET /greet?name=...&birth_year=..."
    }))
}

async fn greet(Query(q): Query<GreetQuery>) -> Json<GreetResponse> {
    let age = Utc::now().year() - q.birth_year;
    Json(GreetResponse {
        greeting: format!("Hello, {}!", q.name),
        age_confirmation: format!("You are {} years old.", age),
    })
}
