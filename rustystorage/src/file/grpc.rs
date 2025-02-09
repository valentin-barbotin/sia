use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{
    Request,
    Response,
    Status, Streaming
};
use log::{error, warn, info, debug, trace, LevelFilter};

use std::pin::Pin;

use file::{
    file_service_server::{
        FileService,
    },
    DownloadRequest,
    DownloadResponse,
    UploadRequest,
    UploadResponse,
    UpdateRequest,
    UpdateResponse,
    DeleteRequest,
    DeleteResponse
};

use super::file::{
    check_if_exists,
    download,
    delete,
    upload,
    generate_uuid
};

use function_name::named;

pub mod file {
    include!(concat!(env!("OUT_DIR"), "/file.v1.rs"));
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("file_descriptor");
}

#[derive(Debug, Default)]
pub struct FileSvc {}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<DownloadResponse, Status>> + Send>>;
type ResponseResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl FileService for FileSvc {
    type GetFileStream = ResponseStream;

    #[named]
    async fn upload_file(
        &self,
        request: Request<Streaming<UploadRequest>>,
    ) -> Result<Response<UploadResponse>, Status> {
        let mut body = request.into_inner();
        let mut out = Vec::new();

        while let Some(result) = body.next().await {
            match result {
                Ok(v) => {
                    let mut dat = v.message.to_owned();
                    out.append(&mut dat);
                }
                Err(err) => {
                    return Err(Status::internal(err.to_string()));
                }
            }
        }

        let uuid = generate_uuid().await;

        match upload(out, &uuid).await {
            Ok(res) => {
                info!("[{}] -- Upload file done", function_name!());
            },
            Err(e) => {
                error!("[{}] -- Upload error: {}", function_name!(), e);
                return Err(Status::internal(e.to_string()));
            },
        };

        let response = UploadResponse {
            uuid
        };
        Ok(Response::new(response))
    }

    #[named]
    async fn update_file(
        &self,
        request: Request<Streaming<UpdateRequest>>,
    ) -> Result<Response<UpdateResponse>, Status> {
        let request_metadata = request.metadata();
        let uuid = request_metadata.get("uuid").unwrap().to_str().unwrap().to_string();
        let mut body = request.into_inner();
        let mut out = Vec::new();

        while let Some(result) = body.next().await {
            match result {
                Ok(v) => {
                    let mut dat = v.message.as_bytes().to_owned();
                    out.append(&mut dat);
                }
                Err(err) => {
                    return Err(Status::internal(err.to_string()));
                }
            }
        }

        match upload(out, &uuid).await {
            Ok(res) => {
                info!("[{}] -- Upload file done", function_name!());
            },
            Err(e) => {
                error!("[{}] -- Upload error: {}", function_name!(), e);
                return Err(Status::internal(e.to_string()));
            },
        };

        let response = UpdateResponse {};
        Ok(Response::new(response))
    }

    #[named]
    async fn get_file(
        &self,
        request: Request<DownloadRequest>,
    ) -> ResponseResult<Self::GetFileStream> {

        let body = request.into_inner();

        let filepath = body.path;

        if filepath.is_empty() {
            return Err(Status::not_found("Empty path".to_string()));
        }

        let data = download(&filepath).await;
        if data.is_err() {
            error!("[{}] -- Download error, {}", function_name!(), data.err().unwrap());
            return Err(Status::not_found("Download error".to_string()));
        }
        let data = data.unwrap();

        info!("[{}] -- Download file {} - Status ({})", function_name!(), filepath, data.status().as_str());

        let bytes = match data.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("[{}] -- {}", function_name!(), e.to_string());
                return Err(Status::internal(e.to_string()));
            }
        };

        let data_vec = bytes.to_vec();

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            let mut stream = data_vec.chunks(1024);

            while let Some(message) = stream.next() {
                let item = DownloadResponse {
                    message: message.to_vec()
                };

                match tx.send(Ok(item)).await {
                    Ok(_) => {

                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        let out_stream = ReceiverStream::new(rx);
        let output = Box::pin(out_stream) as Self::GetFileStream;

        let res = Response::new(
            output
        );

        Ok(res)
    }
    
    #[named]
    async fn delete_file(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let body = request.into_inner();
        let uuid = body.uuid;

        match delete(&uuid).await {
            Ok(res) => {
                info!("[{}] -- Delete file done", function_name!());
            },
            Err(e) => {
                error!("[{}] -- Delete error: {}", function_name!(), e);
                return Err(Status::internal(e.to_string()));
            },
        };

        let response = DeleteResponse {};
        Ok(Response::new(response))
    }
}
