use actix_files as actix_fs; // Rename actix_files
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::fs; // Keep the std::fs name

// Serve the pre-built index.html for the root route
async fn render_index() -> impl Responder {
    let html = fs::read_to_string("/dist/index.html") // Updated path
        .unwrap_or_else(|_| "Error: index.html not found".to_string());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Serve index.html at the root
            .route("/", web::get().to(render_index))
            // Serve all static files from the dist folder
            .service(actix_fs::Files::new("/", "/dist").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
