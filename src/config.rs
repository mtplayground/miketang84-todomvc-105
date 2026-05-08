use std::{env, net::SocketAddr};

pub struct RuntimeEnv {
    pub database_url: Option<String>,
    pub site_addr: Option<SocketAddr>,
}

impl RuntimeEnv {
    pub fn load() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(Box::new(error)),
        }

        Ok(Self {
            database_url: optional_env("DATABASE_URL")?,
            site_addr: optional_env("LEPTOS_SITE_ADDR")?
                .map(|value| value.parse())
                .transpose()?,
        })
    }
}

fn optional_env(key: &str) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    match env::var(key) {
        Ok(value) if value.trim().is_empty() => Ok(None),
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(error) => Err(Box::new(error)),
    }
}
