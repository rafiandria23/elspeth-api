use serde::Deserialize;

use crate::core::error::Result;

//
// API configurations
//

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiMode {
    #[default]
    Development,
    Test,
    Production,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ApiConfig {
    pub mode: ApiMode,

    pub host: String,
    pub port: u16,

    pub cookie_secure: bool,
    pub cookie_secret: String,

    pub auth_session_ttl_days: u32,
    pub oauth_encryption_key: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            mode: ApiMode::default(),

            host: "127.0.0.1".into(),
            port: 3000,

            cookie_secure: false,
            cookie_secret: "elspeth".into(),

            auth_session_ttl_days: 365,
            oauth_encryption_key: "elspeth".into(),
        }
    }
}

//
// Database configurations
//

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,

    pub user: Option<String>,
    pub password: Option<String>,

    pub name: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 5432,

            user: None,
            password: None,

            name: "elspeth".into(),
        }
    }
}

impl DatabaseConfig {
    pub fn url(&self) -> String {
        let mut url = String::from("postgresql://");

        match (&self.user, &self.password) {
            (Some(user), Some(password)) => {
                url.push_str(user);
                url.push(':');
                url.push_str(password);
                url.push('@');
            }
            (Some(user), None) => {
                url.push_str(user);
                url.push('@');
            }
            (None, Some(password)) => {
                url.push(':');
                url.push_str(password);
                url.push('@');
            }
            _ => {}
        }

        url.push_str(&self.host);
        url.push(':');
        url.push_str(&self.port.to_string());

        url
    }
}

//
// JWT configurations
//

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "elspeth".into(),
            issuer: "http://127.0.0.1:3000".into(),
            audience: "http://127.0.0.1:3000".into(),
        }
    }
}

//
// Redis configurations
//

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,

    pub user: Option<String>,
    pub password: Option<String>,

    pub index: Option<u8>,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 6379,
            user: None,
            password: None,
            index: None,
        }
    }
}

impl RedisConfig {
    pub fn url(&self) -> String {
        let mut url = String::from("redis://");

        match (&self.user, &self.password) {
            (Some(user), Some(password)) => {
                url.push_str(user);
                url.push(':');
                url.push_str(password);
                url.push('@');
            }
            (Some(user), None) => {
                url.push_str(user);
                url.push('@');
            }
            (None, Some(password)) => {
                url.push(':');
                url.push_str(password);
                url.push('@');
            }
            _ => {}
        }

        url.push_str(&self.host);
        url.push(':');
        url.push_str(&self.port.to_string());

        if let Some(index) = self.index {
            url.push('/');
            url.push_str(&index.to_string());
        }

        url
    }
}

//
// Telemetry configurations
//

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OtlpProtocol {
    #[default]
    Grpc,
    Http,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct TelemetryConfig {
    pub service_name: Option<String>,

    pub otlp_endpoint: Option<String>,
    pub otlp_protocol: Option<OtlpProtocol>,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            service_name: Some("elspeth-api".into()),

            otlp_endpoint: None,
            otlp_protocol: None,
        }
    }
}

impl TelemetryConfig {
    pub fn resolved_service_name(&self) -> String {
        self.service_name
            .clone()
            .or_else(|| std::env::var("OTEL_SERVICE_NAME").ok())
            .unwrap_or_else(|| "elspeth-api".into())
    }

    pub fn resolved_otlp_endpoint(&self) -> Option<String> {
        self.otlp_endpoint
            .clone()
            .or_else(|| std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok())
    }

    pub fn resolved_otlp_protocol(&self) -> OtlpProtocol {
        if let Some(protocol) = &self.otlp_protocol {
            return protocol.clone();
        }

        if let Ok(env_protocol) = std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL") {
            match env_protocol.to_lowercase().as_str() {
                "http/protobuf" | "http/json" | "http" => return OtlpProtocol::Http,
                "grpc" => return OtlpProtocol::Grpc,
                _ => {}
            }
        }

        if let Some(endpoint) = &self.resolved_otlp_endpoint() {
            if endpoint.contains(":4318") {
                return OtlpProtocol::Http;
            }
        }

        OtlpProtocol::Grpc
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct WebConfig {
    pub url: String,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            url: "http://127.0.0.1:4000".into(),
        }
    }
}

//
// Configuration loader
//

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub api: ApiConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub redis: RedisConfig,
    pub telemetry: TelemetryConfig,
    pub web: WebConfig,
}

impl Config {
    pub fn load(&self) -> Result<Self> {
        dotenvy::dotenv().ok();

        let api_mode = std::env::var("API__MODE").unwrap_or_else(|_| "development".into());

        match api_mode.to_lowercase().as_str() {
            "test" => {
                dotenvy::from_filename_override(".env.test").ok();
                dotenvy::from_filename_override("env.test.local").ok();
            }
            _ => {
                dotenvy::from_filename_override(".env.local").ok();
            }
        }

        let config = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
