use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

use crate::{config::DatabaseConfig, core::error::Result};

pub type DatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DatabasePool {
    inner: Pool<ConnectionManager<PgConnection>>,
}

impl DatabasePool {
    pub fn new(config: &DatabaseConfig) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(&config.url());
        let inner = Pool::builder().max_size(120).build(manager)?;

        Ok(Self { inner })
    }

    pub fn get(&self) -> Result<DatabaseConnection> {
        let conn = self.inner.get()?;

        Ok(conn)
    }

    pub fn health_check(&self) -> Result<()> {
        let mut conn = self.get()?;

        diesel::sql_query("SELECT 1").execute(&mut conn)?;

        Ok(())
    }
}
