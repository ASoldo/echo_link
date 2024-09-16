use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod config;
mod handlers;
mod models;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logging (optional)
    env_logger::init();

    // Get the server address from environment variables or default to localhost:8080
    let server_address =
        env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // Establish database connection pool
    let pool = config::db::establish_connection();

    println!("Starting server at {}", &server_address);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Configure routes
            .configure(routes::init)
    })
    .bind(&server_address)?
    .run()
    .await
}
