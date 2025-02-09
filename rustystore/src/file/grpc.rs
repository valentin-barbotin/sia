use tonic::{
    Request,
    Response,
    Status
};
use log::{error, warn, info, debug, trace, LevelFilter};

use function_name::named;

pub mod storefile {
    include!(concat!(env!("OUT_DIR"), "/storefile.v1.rs"));
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tag_descriptor");
}

use storefile::{
    storefile_service_server::{
        StorefileService,
        StorefileServiceServer
    },
    InsertRequest,
    InsertResponse,
    GetRequest,
    GetResponse,
    DeleteRequest,
    DeleteResponse,
    ListRequest,
    ListResponse
};

use super::controllers::FileInfo;

#[derive(Debug, Default)]
pub struct StorefileSvc {}

#[tonic::async_trait]
impl StorefileService for StorefileSvc {
    #[named]
    async fn insert(
        &self,
        request: Request<InsertRequest>,
    ) -> Result<Response<InsertResponse>, Status> {

        let body = request.into_inner();
        let file_name: String = body.name;
        let file_identifier: String = body.identifier;
        let file_size: i64 = body.size;
        let file_type: String = body.mime_type;
        let file_tags: Vec<String> = body.tags;

        if file_name.trim().is_empty() {
            warn!("[{}] - File name is empty", function_name!());
            return Err(Status::invalid_argument("File name cannot be empty"));
        }

        if file_identifier.trim().is_empty() {
            warn!("[{}] - File identifier is empty", function_name!());
            return Err(Status::invalid_argument("File identifier cannot be empty"));
        }

        if file_size < 0 {
            warn!("[{}] - File size is negative", function_name!());
            return Err(Status::invalid_argument("File size cannot be negative"));
        }

        if file_type.trim().is_empty() {
            warn!("[{}] - File type is empty", function_name!());
            return Err(Status::invalid_argument("File type cannot be empty"));
        }

        let file_info: FileInfo = FileInfo {
            name: file_name.clone(),
            identifier: file_identifier.clone(),
            size: file_size,
            mime_type: file_type.clone(),
            tags: file_tags.clone()
        };

        match super::file::insert(&file_info) {
            Ok(file) => {
                info!("[{}] - File inserted ({})", function_name!(), file_name);
                let response = InsertResponse {
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't insert file, {}", function_name!(), e);
                Err(Status::internal("Can't insert file"))
            }
        }
    }

    #[named]
    async fn get(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetResponse>, Status> {

        let body = request.into_inner();
        let file_identifier: String = body.id;
        if file_identifier.trim().is_empty() {
            warn!("[{}] - File identifier is empty", function_name!());
            return Err(Status::invalid_argument("File identifier cannot be empty"));
        }

        match super::file::get(&file_identifier) {
            Ok(file) => {
                info!("[{}] - File retrieved ({})", function_name!(), file_identifier);

                let body: storefile::File = storefile::File {
                    id: file.id,
                    name: file.name,
                    identifier: file.identifier,
                    size: file.size,
                    mime_type: file.mime_type,
                    created_at: file.created_at.to_string(),
                    updated_at: file.updated_at.to_string(),
                };

                let response = GetResponse {
                    file: Some(body)
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't retrieve file, {}", function_name!(), e);
                Err(Status::internal("Can't retrieve file"))
            }
        }
    }

    #[named]
    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {

        let body = request.into_inner();
        let file_identifier: String = body.id;
        if file_identifier.trim().is_empty() {
            warn!("[{}] - File identifier is empty", function_name!());
            return Err(Status::invalid_argument("File identifier cannot be empty"));
        }

        match super::file::remove(&file_identifier) {
            Ok(file) => {
                info!("[{}] - File deleted ({})", function_name!(), file_identifier);
                let response = DeleteResponse {
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't delete file, {}", function_name!(), e);
                Err(Status::internal("Can't delete file"))
            }
        }
    }

    #[named]
    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListResponse>, Status> {

        match super::file::get_all() {
            Ok(files) => {
                info!("[{}] - File list retrieved", function_name!());
                let mut response = ListResponse {
                    files: Vec::new()
                };
                for file in files {
                    response.files.push(storefile::File {
                        id: file.id,
                        name: file.name,
                        identifier: file.identifier,
                        size: file.size,
                        mime_type: file.mime_type,
                        created_at: file.created_at.to_string(),
                        updated_at: file.updated_at.to_string(),
                    });
                }
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't retrieve file list, {}", function_name!(), e);
                Err(Status::internal("Can't retrieve file list"))
            }
        }
    }
}
