use std::{env, time::Duration};

use lazy_static::lazy_static;

/*
    Static variables that are initialized with the environment variables.
    lazy_static != static, (static => compile time, lazy_static => runtime)
*/
lazy_static! {
    pub static ref APIKEY: String = env::var("APIKEY").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("APIKEY"))
    });

    pub static ref PORT: u16 = env::var("PORT").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("PORT"))
    }).parse().unwrap_or_else(|e| {
        panic!("PORT is not a valid number: {}", e)
    });

    pub static ref PORT_GRPC: u16 = env::var("PORT_GRPC").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("PORT_GRPC"))
    }).parse().unwrap_or_else(|e| {
        panic!("PORT is not a valid number: {}", e)
    });

    pub static ref RUST_ENV: String = env::var("RUST_ENV").unwrap_or("info".to_string());

    /// Database
    pub static ref DB_PASSWORD: String = env::var("DB_PASSWORD").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("DB_PASSWORD"));
    });
    pub static ref DB_USERNAME: String = env::var("DB_USERNAME").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("DB_USERNAME"));
    });
    pub static ref DB_HOST: String = env::var("DB_HOST").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("DB_HOST"));
    });
    pub static ref DB_PORT: u16 = env::var("DB_PORT").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("DB_PORT"));
    }).parse().unwrap_or_else(|e| {
        panic!("Can't parse DB_PORT {}", e);
    });

    pub static ref DB_NAME: String = env::var("DB_NAME").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("DB_NAME"));
    });
    pub static ref DB_PARAMS: String = env::var("DB_PARAMS").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("DB_PARAMS"));
    });

    pub static ref DB_MIN_CONNECTIONS: u32 = env::var("DB_MIN_CONNECTIONS").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("DB_MIN_CONNECTIONS"));
    }).parse().unwrap_or_else(|e| {
        panic!("Can't parse DB_MIN_CONNECTIONS {}", e);
    });

    pub static ref DB_MAX_CONNECTIONS: u32 = env::var("DB_MAX_CONNECTIONS").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("DB_MAX_CONNECTIONS"));
    }).parse().unwrap_or_else(|e| {
        panic!("Can't parse DB_MAX_CONNECTIONS {}", e);
    });

    pub static ref DB_CONNECTION_TIMEOUT: Duration = Duration::from_secs(
        env::var("DB_CONNECTION_TIMEOUT").unwrap_or_else(|_e| {
            panic!("{}", var_not_defined("DB_CONNECTION_TIMEOUT"));
        }).parse().unwrap_or_else(|e| {
            panic!("Can't parse DB_CONNECTION_TIMEOUT {}", e);
        })
    );
}

/*
    Use this function to get a nice error message when a variable is not defined.
*/
fn var_not_defined(var: &str) -> String {
    format!("{} is not defined in the environment", var)
}

/*
    Check if all variables are defined. If not, panic.
*/
pub fn check_vars() {
    lazy_static::initialize(&APIKEY);
    lazy_static::initialize(&PORT);
    lazy_static::initialize(&PORT_GRPC);
    lazy_static::initialize(&RUST_ENV); // don't panic if RUST_ENV is not defined

    lazy_static::initialize(&DB_PASSWORD);
    lazy_static::initialize(&DB_USERNAME);
    lazy_static::initialize(&DB_HOST);
    lazy_static::initialize(&DB_PORT);
    lazy_static::initialize(&DB_NAME);
    lazy_static::initialize(&DB_PARAMS);
    lazy_static::initialize(&DB_MIN_CONNECTIONS);
    lazy_static::initialize(&DB_MAX_CONNECTIONS);
    lazy_static::initialize(&DB_CONNECTION_TIMEOUT);
}
