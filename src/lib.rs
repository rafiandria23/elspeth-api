pub mod config;
pub mod core;
pub mod modules;

use actix_web::{App, HttpServer, dev::Server};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::{
    config::Config,
    core::{database::pool::DatabasePool, error::Result, redis::RedisClient},
};

#[derive(Clone)]
pub struct ApiState {
    pub config: Config,
    pub database: DatabasePool,
    pub redis: RedisClient,
}

pub fn run(listener: TcpListener, state: ApiState) -> Result<Server> {
    let state_data = actix_web::web::Data::new(state);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(state_data.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
