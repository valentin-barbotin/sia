use s3::{
    Bucket,
    Region,
    creds::{
        Credentials
    },
    error::{
        S3Error
    }, BucketConfiguration
};
use log::{error, warn, info, debug, trace, LevelFilter};
use function_name::named;

use crate::local_env::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref CREDENTIALS: Credentials = Credentials::new(Some(&MINIO_ACCESS_KEY), Some(&MINIO_SECRET_KEY), None, None, None).unwrap();
}

#[named]
pub fn get_bucket() -> Result<Bucket, S3Error> {
    debug!("[{}] -- Get bucket instance {}", function_name!(), *MINIO_BUCKET);
    
    let credentials = CREDENTIALS.to_owned();

    let bucket = Bucket::new(
        &MINIO_BUCKET,
        Region::Custom {
            region: MINIO_BUCKET.to_string(),
            endpoint: format!("http://{}:{}", *MINIO_HOST, *MINIO_PORT),
        },
        credentials
    )?
    .with_path_style();

    Ok(bucket)
}

#[named]
pub async fn create_bucket() -> Result<Bucket, S3Error> {
    info!("[{}] -- Bucket creation.. {}", function_name!(), *MINIO_BUCKET);
    let credentials = CREDENTIALS.to_owned();

    let config = BucketConfiguration::default();
    let create_bucket_res = Bucket::create_with_path_style(
        &MINIO_BUCKET,
        Region::Custom {
            region: MINIO_BUCKET.to_string(),
            endpoint: format!("http://{}:{}", *MINIO_HOST, *MINIO_PORT),
        },
        credentials,
        config
    ).await?;

    info!("[{}] -- Bucket creation done - {} - Code: {}", function_name!(), create_bucket_res.response_text, create_bucket_res.response_code);

    Ok(create_bucket_res.bucket)
}
