use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub rate_limit_requests: u64,
    pub rate_limit_duration: u64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
            
            rate_limit_requests: env::var("RATE_LIMIT_REQUEST")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .expect("RATE_LIMIT_REQUEST must be a number"),
                
            rate_limit_duration: env::var("RATE_LIMIT_DURATION")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .expect("RATE_LIMIT_DURATION must be a number"),
        }
    }
} 