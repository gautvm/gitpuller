use actix_web::{web, App, HttpServer};
use std::process::Command;

async fn handle_push_event(payload: String) -> String {
    println!("Received new event: {}", payload);
    let output = Command::new("git")
        .arg("pull")
        .output()
        .expect("Error running git pull");

    println!("command output: {}", String::from_utf8_lossy(&output.stdout));

    "Push event received".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    HttpServer::new(|| App::new().route("/webhook", web::post().to(handle_push_event)))
        .bind(addr)?
        .run()
        .await
}
