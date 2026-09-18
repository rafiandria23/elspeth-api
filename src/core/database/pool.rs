use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

use crate::{config::DatabaseConfig, core::error::Result};

pub type DatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DatabasePool {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DatabasePool {
    pub fn new(config: &DatabaseConfig) -> Result<Self> {
        let max_size = config.resolved_pool_max_size();
        let min_idle = config.resolved_pool_min_idle()?;

        let manager = ConnectionManager::<PgConnection>::new(&config.url());
        let pool = Pool::builder()
            .max_size(max_size)
            .min_idle(Some(min_idle))
            .build(manager)?;

        tracing::info!(
            pool_max_size = max_size,
            pool_min_idle = min_idle,
            "Database connection pool initialized"
        );

        Ok(Self { pool })
    }

    pub fn get(&self) -> Result<DatabaseConnection> {
        let conn = self.pool.get()?;

        Ok(conn)
    }

    pub fn health_check(&self) -> Result<()> {
        let mut conn = self.get()?;

        diesel::sql_query("SELECT 1").execute(&mut conn)?;

        Ok(())
    }
}
