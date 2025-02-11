use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::net::SocketAddr;
mod infrastructure;
mod application;
mod domain;
mod presentation;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    // Load configuration
    let config = infrastructure::config::app::AppConfig::from_env();
    
    // Setup database connection pool
    let pool = infrastructure::config::database::establish_connection_pool();
    
    // Setup JWT service
    let jwt_service = infrastructure::auth::jwt::JwtService::new();
    
    // Create and run server
    let server = infrastructure::server::Server::new(
        config,
        pool,
        jwt_service,
    );
    
    server.run().await;
}
