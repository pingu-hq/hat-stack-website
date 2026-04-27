use axum::response::{Html, IntoResponse, Response};
use axum::http::StatusCode;
use std::fs;

pub async fn about_me_page() -> Response {
    match fs::read_to_string("templates/full_page/about_me.html") {
        Ok(html) => Html(html).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}


