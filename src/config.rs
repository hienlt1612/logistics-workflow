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

    #[test]
    fn test_env_var_overrides() {
        // Set env vars to override default config values
        std::env::set_var("LW_DB_HOST", "override-host");
        std::env::set_var("LW_DB_PORT", "9999");
        std::env::set_var("LW_DB_NAME", "override_db");
        std::env::set_var("LW_DB_USER", "override_user");
        std::env::set_var("LW_DB_PASSWORD", "override_pass");

        // Create a minimal config file with defaults so load() reads it first
        // then applies overrides from env vars.
        // Since config.toml exists in the project root, load() will read it,
        // then apply our env var overrides.
        let config = Config::load().expect("load config with env overrides");

        assert_eq!(config.host, "override-host");
        assert_eq!(config.port, 9999);
        assert_eq!(config.db_name, "override_db");
        assert_eq!(config.user, "override_user");
        assert_eq!(config.password, "override_pass");

        // Clean up env vars
        std::env::remove_var("LW_DB_HOST");
        std::env::remove_var("LW_DB_PORT");
        std::env::remove_var("LW_DB_NAME");
        std::env::remove_var("LW_DB_USER");
        std::env::remove_var("LW_DB_PASSWORD");
    }

    #[test]
    fn test_api_token_from_env() {
        std::env::set_var("LW_API_TOKEN", "my-secret-token");

        // Load config — api_token should be picked up from env
        let config = Config::load().expect("load config with api token");
        assert_eq!(config.api_token, Some("my-secret-token".to_string()));

        // Test empty string yields None
        std::env::set_var("LW_API_TOKEN", "");
        let config2 = Config::load().expect("load config with empty api token");
        assert_eq!(config2.api_token, None);

        std::env::remove_var("LW_API_TOKEN");
    }
}
