use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub frontend_url: String,
    pub port: u16,
    pub rust_log: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set".to_string())?;
        
        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| "JWT_SECRET must be set".to_string())?;
        
        if jwt_secret.len() < 32 {
            return Err("JWT_SECRET must be at least 32 characters".to_string());
        }

        let jwt_expiry_hours = std::env::var("JWT_EXPIRY_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .map_err(|_| "JWT_EXPIRY_HOURS must be a valid number".to_string())?;

        let frontend_url = std::env::var("FRONTEND_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| "PORT must be a valid number".to_string())?;

        let rust_log = std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());

        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expiry_hours,
            frontend_url,
            port,
            rust_log,
        })
    }

    pub fn global() -> &'static Config {
        CONFIG.get().expect("Config not initialized")
    }

    pub fn init() -> Result<(), String> {
        let config = Self::from_env()?;
        CONFIG.set(config).map_err(|_| "Config already initialized".to_string())?;
        Ok(())
    }
}
