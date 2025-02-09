// use s3::{request_trait::ResponseData, error::S3Error, serde_types::HeadObjectResult};
use std::pin::Pin;
use function_name::named;
// use crate::s3::client;
use log::{error, warn, info, debug, trace, LevelFilter};
use reqwest;
use s3::{error::S3Error, request::ResponseData};

use crate::local_env::*;

#[named]
pub async fn check_if_exists(uuid: &str) -> bool {
    let url = format!("{}://{}:{}@{}:{}/api/worker/object/{uuid}?bucket={}", *SIA_RENTERD_PROTOCOL, *SIA_RENTERD_USER, *SIA_RENTERD_PASSWORD, *SIA_RENTERD_HOST, *SIA_RENTERD_PORT, *SIA_RENTERD_BUCKET);
    let client = reqwest::Client::new();

    match client.head(&url).send().await {
        Ok(res) => {
            return res.status().is_success();
        },
        Err(e) => {
            error!("[{}] -- Head object error {}", function_name!(), e);
            panic!("{}", e)
        }
    }
}

#[named]
pub async fn download(filepath: &str) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("{}://{}:{}@{}:{}/api/worker/object/{filepath}?bucket={}", *SIA_RENTERD_PROTOCOL, *SIA_RENTERD_USER, *SIA_RENTERD_PASSWORD, *SIA_RENTERD_HOST, *SIA_RENTERD_PORT, *SIA_RENTERD_BUCKET);
    let client = reqwest::Client::new();

    return client.get(&url).send().await;
}

#[named]
pub async fn delete(filepath: &str) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("{}://{}:{}@{}:{}/api/worker/object/{filepath}?bucket={}", *SIA_RENTERD_PROTOCOL, *SIA_RENTERD_USER, *SIA_RENTERD_PASSWORD, *SIA_RENTERD_HOST, *SIA_RENTERD_PORT, *SIA_RENTERD_BUCKET);
    let client = reqwest::Client::new();

    return client.delete(&url).send().await;
}

#[named]
pub async fn upload(data: Vec<u8>, uuid: &str) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("{}://{}:{}@{}:{}/api/worker/object/{uuid}?bucket={}", *SIA_RENTERD_PROTOCOL, *SIA_RENTERD_USER, *SIA_RENTERD_PASSWORD, *SIA_RENTERD_HOST, *SIA_RENTERD_PORT, *SIA_RENTERD_BUCKET);
    let client = reqwest::Client::new();

    return client.put(&url).body(data).send().await;
}

#[named]
pub async fn generate_uuid() -> String {
    // generate uuid
    let mut uuid;
    // let mut uuid = String::new();
    loop {
        uuid = uuid::Uuid::new_v4().to_string();
        if !check_if_exists(&uuid).await {
            break;
        }
        error!("[{}] -- UUID already exists, generating new one", function_name!());
    }

    return uuid;
}
