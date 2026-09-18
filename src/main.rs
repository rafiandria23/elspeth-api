use elspeth_api::{
    ApiState,
    config::Config,
    core::{
        database::pool::DatabasePool, error::Result, redis::RedisClient, telemetry::ApiTelemetry,
    },
};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    let _telemetry = ApiTelemetry::init(&config.telemetry)?;

    let database = DatabasePool::new(&config.database)?;
    let redis = RedisClient::new(&config.redis).await?;

    let state = ApiState {
        config: config.clone(),
        database,
        redis,
    };

    let address = format!("{}:{}", config.api.host, config.api.port);
    let listener = TcpListener::bind(&address)?;

    tracing::info!("Elspeth API is running on http://{address}");

    elspeth_api::run(listener, state)?.await?;

    Ok(())
}
