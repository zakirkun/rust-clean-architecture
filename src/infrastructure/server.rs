use std::net::SocketAddr;
use axum::{
    routing::{get, post},
    Router,
    middleware,
    http::{Method, HeaderValue},
    Json,
};
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    limit::RateLimitLayer,
};
use tracing::Level;
use tower::limit::RateLimit;
use std::time::Duration;
use serde_json::json;
use crate::{
    infrastructure::config::{database::DbPool, app::AppConfig},
    infrastructure::auth::jwt::JwtService,
    application::{handlers, middleware::auth::auth_middleware},
    domain::repositories::user_repository::DieselUserRepository,
    application::handlers::users,
};

pub struct Server {
    config: AppConfig,
    db_pool: DbPool,
    jwt_service: JwtService,
}

impl Server {
    pub fn new(config: AppConfig, db_pool: DbPool, jwt_service: JwtService) -> Self {
        Self {
            config,
            db_pool,
            jwt_service,
        }
    }

    fn setup_cors(&self) -> CorsLayer {
        CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(Any)
            .allow_origin(Any)
    }

    fn setup_logging(&self) -> TraceLayer {
        TraceLayer::new_for_http()
            .make_span_with(|request: &axum::http::Request<_>| {
                tracing::span!(
                    Level::INFO,
                    "request",
                    method = %request.method(),
                    uri = %request.uri(),
                )
            })
    }

    async fn health_check() -> Json<serde_json::Value> {
        Json(json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }

    fn create_router(&self) -> Router {
        // Rate limit configuration from env
        let rate_limit = RateLimit::new(
            self.config.rate_limit_requests,
            Duration::from_secs(self.config.rate_limit_duration),
            RateLimitLayer::new(),
        );

        let user_repository = DieselUserRepository::new(self.db_pool.clone());

        // Public routes
        let public_routes = Router::new()
            .route("/auth/login", post(handlers::auth::login::<DieselUserRepository>))
            .route("/auth/register", post(handlers::auth::register::<DieselUserRepository>))
            .route("/health", get(Self::health_check))
            .route("/users", post(users::create_user::<DieselUserRepository>))
            .route("/users", get(users::list_users::<DieselUserRepository>));
        
        // Protected routes
        let protected_routes = Router::new()
            .route("/protected", get(handlers::protected::handler))
            .route(
                "/users/:id",
                get(users::get_user::<DieselUserRepository>)
                    .put(users::update_user::<DieselUserRepository>)
                    .delete(users::delete_user::<DieselUserRepository>),
            )
            .layer(middleware::from_fn_with_state(
                self.jwt_service.clone(),
                auth_middleware,
            ));
        
        // Combine all routes with middleware
        Router::new()
            .merge(public_routes)
            .merge(protected_routes)
            .layer(self.setup_cors())
            .layer(self.setup_logging())
            .layer(rate_limit)
            .with_state(self.db_pool.clone())
            .with_state(self.jwt_service.clone())
    }

    pub async fn run(&self) {
        // Setup tracing
        tracing_subscriber::fmt()
            .with_target(false)
            .compact()
            .init();

        let addr = SocketAddr::from(([127, 0, 0, 1], self.config.port));
        tracing::info!("Server running on http://{}", addr);

        axum::Server::bind(&addr)
            .serve(self.create_router().into_make_service())
            .await
            .unwrap();
    }
} 