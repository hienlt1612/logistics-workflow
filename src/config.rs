use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub user: String,
    pub password: String,
}

impl Config {
    /// Load config from current dir, fallback to ~/.config/logistics-workflow/
    pub fn load() -> Result<Self, anyhow::Error> {
        let paths = vec![
            PathBuf::from("config.toml"),
            dirs_config().join("config.toml"),
        ];

        for path in &paths {
            if path.exists() {
                let content = std::fs::read_to_string(path)?;
                return Ok(toml::from_str(&content)?);
            }
        }

        anyhow::bail!("config.toml not found. Copy config.toml.example to config.toml and edit.");
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
        };
        assert_eq!(
            config.db_url(),
            "postgres://mim_dev:test123@127.0.0.1:5432/logistics_workflow"
        );
    }
}
