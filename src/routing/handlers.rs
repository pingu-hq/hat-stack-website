use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Html};
use std::fs;

// Simple handler: serve index.html from templates/
pub async fn index() -> Response {
    match fs::read_to_string("templates/index.html") {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template not found").into_response()
    }
}

// HTMX partial handler: return fragment for dynamic loading
pub async fn users_partial() -> Response {
    match fs::read_to_string("templates/partials/user-list.html") {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Partial not found").into_response()
    }
}