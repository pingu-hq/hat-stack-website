use axum::{routing::get, Router, serve};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod routing;  // Declares routing module

#[tokio::main]
async fn main() {
    println!("🚀 Starting Rust SSR server... Ready for HTMX!");

    // Serve HTML templates from /templates
    println!("🚀 Starting Rust SSR server... Ready for HTMX!");

    // Serve HTML templates from /templates
    let templates = ServeDir::new("templates")
        .append_index_html_on_directories(true);

    // Serve static assets (CSS, JS, images) from /static
    let static_files = ServeDir::new("static");

    // Router: dynamic routes + static file services
    let app = Router::new()
        // Dynamic HTMX routes (handlers in src/routing/handlers.rs)
        .route("/", get(routing::handlers::index))
        .route("/users", get(routing::handlers::users_partial))
        // Static file services
        .nest_service("/static", static_files)
        // Fallback: serve templates/ for everything else
        .fallback_service(templates);

    let listener = TcpListener::bind("127.0.0.1:8001")
        .await
        .unwrap();
    
    println!("🌸 Server running on http://127.0.0.1:8001");
    println!("✨ Try: http://127.0.0.1:8001/ or /users");
    
    serve(listener, app).await.unwrap();
}