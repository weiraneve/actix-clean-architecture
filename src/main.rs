use std::sync::Arc;
use std::{env, io};

use actix_web::HttpServer;
use sqlx::MySqlPool;

use actix_clean_architecture::creat_app::create_app;

#[actix_web::main]
async fn main() -> io::Result<()> {
    init_environment();
    let server_addr = env::var(SERVER_ADDR).expect(SERVER_ADDR_NOT_SET_MSG);
    let database_url = env::var(DATABASE_URL).expect(DATABASE_URL_NOT_SET_MSG);
    let pool = init_database(database_url).await;

    log::info!("{}{}", STARTING_SERVER_LOG, server_addr);

    HttpServer::new(move || create_app(pool.clone()))
        .bind(server_addr)?
        .run()
        .await
}

fn init_environment() {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(LOGGER_FILTER_LEVEL));
}

async fn init_database(database_url: String) -> Arc<MySqlPool> {
    let connection = MySqlPool::connect(&database_url).await.unwrap();
    Arc::new(connection)
}

const SERVER_ADDR: &str = "SERVER_ADDR";
const DATABASE_URL: &str = "DATABASE_URL";
const SERVER_ADDR_NOT_SET_MSG: &str = "SERVER_ADDR is not set in .env file";
const DATABASE_URL_NOT_SET_MSG: &str = "DATABASE_URL must be set";
const STARTING_SERVER_LOG: &str = "starting HTTP server at ";
const LOGGER_FILTER_LEVEL: &str = "DEBUG";
