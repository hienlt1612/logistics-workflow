use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub user: String,
    pub password: String,
    /// Optional API token for write-protection. If set, POST/PATCH/DELETE require
    /// `Authorization: Bearer <token>` header. GET requests remain public.
    pub api_token: Option<String>,
}

impl Config {
    /// Load config from config.toml, with env var overrides for Docker.
    /// Env vars: LW_DB_HOST, LW_DB_PORT, LW_DB_NAME, LW_DB_USER, LW_DB_PASSWORD, LW_API_TOKEN
    pub fn load() -> Result<Self, anyhow::Error> {
        let paths = vec![
            PathBuf::from("config.toml"),
            dirs_config().join("config.toml"),
        ];

        // Try file first, fall back to env-only
        let mut config = if let Some(path) = paths.iter().find(|p| p.exists()) {
            let content = std::fs::read_to_string(path)?;
            toml::from_str::<Config>(&content)?
        } else {
            Config {
                host: "127.0.0.1".into(),
                port: 5432,
                db_name: "logistics_workflow".into(),
                user: "lw_user".into(),
                password: "change-me".into(),
                api_token: None,
            }
        };

        // Env var overrides (for Docker / 12-factor)
        if let Ok(v) = std::env::var("LW_DB_HOST") { config.host = v; }
        if let Ok(v) = std::env::var("LW_DB_PORT") { config.port = v.parse().unwrap_or(5432); }
        if let Ok(v) = std::env::var("LW_DB_NAME") { config.db_name = v; }
        if let Ok(v) = std::env::var("LW_DB_USER") { config.user = v; }
        if let Ok(v) = std::env::var("LW_DB_PASSWORD") { config.password = v; }
        if let Ok(v) = std::env::var("LW_API_TOKEN") { config.api_token = if v.is_empty() { None } else { Some(v) }; }

        Ok(config)
    }

    /// Build PostgreSQL connection string
    pub fn db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }
}

fn dirs_config() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/mim".into());
    PathBuf::from(home)
        .join(".config")
        .join("logistics-workflow")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_url_format() {
        let config = Config {
            host: "127.0.0.1".into(),
            port: 5432,
            db_name: "logistics_workflow".into(),
            user: "mim_dev".into(),
            password: "test123".into(),
            api_token: None,
        };
        assert_eq!(
            config.db_url(),
            "postgres://mim_dev:test123@127.0.0.1:5432/logistics_workflow"
        );
    }
}
