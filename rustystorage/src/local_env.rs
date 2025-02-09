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
        panic!("PORT_GRPC is not a valid number: {}", e)
    });

    pub static ref RUST_ENV: String = env::var("RUST_ENV").unwrap_or("info".to_string());

    pub static ref SIA_RENTERD_PORT: u16 = env::var("SIA_RENTERD_PORT").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("SIA_RENTERD_PORT"))
    }).parse().unwrap_or_else(|e| {
        panic!("SIA_RENTERD_PORT is not a valid number: {}", e)
    });

    pub static ref SIA_RENTERD_HOST: String = env::var("SIA_RENTERD_HOST").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("SIA_RENTERD_HOST"))
    });

    pub static ref SIA_RENTERD_USER: String = env::var("SIA_RENTERD_USER").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("SIA_RENTERD_USER"))
    });

    pub static ref SIA_RENTERD_PASSWORD: String = env::var("SIA_RENTERD_PASSWORD").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("SIA_RENTERD_PASSWORD"))
    });

    pub static ref SIA_RENTERD_BUCKET: String = env::var("SIA_RENTERD_BUCKET").unwrap_or_else(|_e| {
        panic!("{}", var_not_defined("SIA_RENTERD_BUCKET"))
    });

    pub static ref SIA_RENTERD_PROTOCOL: String = env::var("SIA_RENTERD_PROTOCOL").unwrap_or("http".to_string());


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

    lazy_static::initialize(&SIA_RENTERD_PROTOCOL);
    lazy_static::initialize(&SIA_RENTERD_HOST);
    lazy_static::initialize(&SIA_RENTERD_PORT);
    lazy_static::initialize(&SIA_RENTERD_USER);
    lazy_static::initialize(&SIA_RENTERD_PASSWORD);
    lazy_static::initialize(&SIA_RENTERD_BUCKET);
}
