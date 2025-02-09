use lazy_static::lazy_static;
use log::{error, warn, info, debug, trace, LevelFilter};

use diesel::{
    pg::PgConnection,
    r2d2::{
        Pool,
        ConnectionManager
    },
};

use super::local_env::*;

#[macro_export]
macro_rules! getConn {
    () => {
        &mut $crate::POOL.get().unwrap()
    };
}

lazy_static! {
    pub static ref POOL: Pool<ConnectionManager<PgConnection>> = {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}{}",
            *DB_USERNAME,
            *DB_PASSWORD,
            *DB_HOST,
            *DB_PORT,
            *DB_NAME,
            *DB_PARAMS
        );

        let manager = ConnectionManager::<PgConnection>::new(database_url);

        info!("Connecting to database...");

        let builder = diesel::r2d2::Pool::builder()
            .test_on_check_out(true)
            .min_idle(Some(*DB_MIN_CONNECTIONS))
            .max_size(*DB_MAX_CONNECTIONS)
            .connection_timeout(*DB_CONNECTION_TIMEOUT)
            .build(manager);

        match builder {
            Ok(pool) => {
                info!("Connected to database");
                pool
            },
            Err(e) => {
                panic!("Can't connect to database: {}", e);
            }
        }
    };
}