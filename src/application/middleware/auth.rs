use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum::headers::{Authorization, Bearer};
use axum::TypedHeader;

use crate::infrastructure::auth::jwt::JwtService;

pub async fn auth_middleware<B>(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(jwt_service): State<JwtService>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let token = bearer.token();
    
    match jwt_service.verify_token(token) {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
} 