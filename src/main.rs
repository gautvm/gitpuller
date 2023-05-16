use std::process::Command;
use actix_web::{web, App, HttpServer};

async fn handle_push_event(payload: String) -> String {
    println!("Received push event");
    let output = Command::new("git").arg("pull").output().expect("Error pulling");

    println!("output: {}", String::from_utf8_lossy(&output.stdout));

    "Push event received".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(handle_push_event))
    })
    .bind(addr)?
    .run()
    .await
}