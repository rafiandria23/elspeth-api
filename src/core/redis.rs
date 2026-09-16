use redis::{AsyncCommands, Client, aio::ConnectionManager};

use crate::{config::RedisConfig, core::error::Result};

#[derive(Clone)]
pub struct RedisClient {
    manager: ConnectionManager,
}

impl RedisClient {
    pub async fn new(config: &RedisConfig) -> Result<Self> {
        let client = Client::open(config.url())?;
        let manager = ConnectionManager::new(client).await?;

        Ok(Self { manager })
    }

    pub fn get(&self) -> ConnectionManager {
        self.manager.clone()
    }

    pub async fn health_check(&self) -> Result<()> {
        let mut conn = self.get();
        let _: String = AsyncCommands::ping(&mut conn).await?;

        Ok(())
    }
}
