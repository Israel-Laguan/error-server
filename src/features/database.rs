use ::core::panic;
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

use crate::server::configuration::env_variables;

pub fn db_pool() -> Pool {
    let conf = env_variables();
    let mut cfg = Config::new();
    cfg.user = Some(conf.postgres_user);
    cfg.password = Some(conf.postgres_password);
    cfg.host = Some(conf.postgres_db_url);
    cfg.port = Some(5432);
    cfg.dbname = Some(conf.postgres_db);
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls);
    match pool {
        Ok(pool) => pool,
        Err(_) => panic!("DB not ready"),
    }
}

pub async fn check_db_is_reachable() -> bool {
    let pool = db_pool();
    let client = pool.get().await;
    match client {
        Ok(_) => true,
        Err(error) => {
            log::error!("{error}");
            false
        },
    }
}
