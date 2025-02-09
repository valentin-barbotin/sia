use tonic::{
    Request,
    Response,
    Status
};
use log::{error, warn, info, debug, trace, LevelFilter};

use function_name::named;

pub mod tag {
    include!(concat!(env!("OUT_DIR"), "/tag.v1.rs"));
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tag_descriptor");
}

use tag::{
    tag_service_server::{
        TagService,
        TagServiceServer
    },
    CreateTagRequest,
    CreateTagResponse,
    RemoveTagRequest,
    RemoveTagResponse,
    LinkFileWithTagRequest,
    LinkFileWithTagResponse,
    GetTagsRequest,
    GetTagsResponse,
    GetFileTagsRequest,
    GetFileTagsResponse,
    RemoveFileTagRequest,
    RemoveFileTagResponse,
    GetFilesTaggedWithRequest,
    GetFilesTaggedWithResponse,
};

use super::super::file::grpc::storefile;

#[derive(Debug, Default)]
pub struct TagSvc {}

#[tonic::async_trait]
impl TagService for TagSvc {
    #[named]
    async fn create_tag(
        &self,
        request: Request<CreateTagRequest>,
    ) -> Result<Response<CreateTagResponse>, Status> {

        let tag_name: String = request.into_inner().name;
        if tag_name.trim().is_empty() {
            warn!("[{}] - Tag name is empty", function_name!());
            return Err(Status::invalid_argument("Tag name cannot be empty"));
        }

        match super::tag::create_tag(&tag_name) {
            Ok(tag) => {
                info!("[{}] - Tag inserted ({})", function_name!(), tag_name);
                let response = CreateTagResponse {
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't create tag, {}", function_name!(), e);
                if e.to_string().contains("duplicate key value") {
                    return Err(Status::invalid_argument("Tag already exists"));
                }
                return Err(Status::internal("Can't create tag"));
            },
        }
    }

    #[named]
    async fn remove_tag(
        &self,
        request: Request<RemoveTagRequest>,
    ) -> Result<Response<RemoveTagResponse>, Status> {

        let tag_name: String = request.into_inner().name;
        if tag_name.trim().is_empty() {
            warn!("[{}] - Tag name is empty", function_name!());
            return Err(Status::invalid_argument("Tag name cannot be empty"));
        }

        match super::tag::remove_tag(&tag_name) {
            Ok(_) => {
                info!("[{}] - Tag removed ({})", function_name!(), tag_name);
                let response = RemoveTagResponse {
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't remove tag, {}", function_name!(), e);
                return Err(Status::internal("Can't remove tag"));
            },
        }
    }

    #[named]
    async fn link_file_with_tag(
        &self,
        request: Request<LinkFileWithTagRequest>,
    ) -> Result<Response<LinkFileWithTagResponse>, Status> {

        let body = request.into_inner();

        let file_id: String = body.file_id;
        let tag_name: String = body.tag_name;
        if file_id.trim().is_empty() {
            warn!("[{}] - File id is empty", function_name!());
            return Err(Status::invalid_argument("File id cannot be empty"));
        }
        if tag_name.trim().is_empty() {
            warn!("[{}] - Tag name is empty", function_name!());
            return Err(Status::invalid_argument("Tag name cannot be empty"));
        }

        match super::tag::link_file_with_tag(&file_id, &tag_name) {
            Ok(_) => {
                info!("[{}] - File linked with tag ({}, {})", function_name!(), file_id, tag_name);
                let response = LinkFileWithTagResponse {
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't link file with tag, {}", function_name!(), e);
                return Err(Status::internal("Can't link file with tag"));
            },
        }
    }

    #[named]
    async fn get_tags(
        &self,
        request: Request<GetTagsRequest>,
    ) -> Result<Response<GetTagsResponse>, Status> {

        match super::tag::get_tags() {
            Ok(list) => {
                info!("[{}] - Got tags", function_name!());
                let mut tags: Vec<tag::Tag> = Vec::new();

                for tag in list {
                    tags.push(tag::Tag {
                        name: tag.name,
                    });
                }

                let response = GetTagsResponse {
                    tags
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't get tags, {}", function_name!(), e);
                return Err(Status::internal("Can't get tags"));
            },
        }
    }

    #[named]
    async fn get_file_tags(
        &self,
        request: Request<GetFileTagsRequest>,
    ) -> Result<Response<GetFileTagsResponse>, Status> {

        let file_id: String = request.into_inner().file_id;
        if file_id.trim().is_empty() {
            warn!("[{}] - File id is empty", function_name!());
            return Err(Status::invalid_argument("File id cannot be empty"));
        }

        match super::tag::get_files_tagged_with(&file_id) {
            Ok(list) => {
                info!("[{}] - Got file tags", function_name!());
                let mut tags: Vec<tag::Tag> = Vec::new();

                for tag in list {
                    tags.push(tag::Tag {
                        name: tag.name,
                    });
                }

                let response = GetFileTagsResponse {
                    tags,
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't get file tags, {}", function_name!(), e);
                return Err(Status::internal("Can't get file tags"));
            },
        }
    }

    #[named]
    async fn remove_file_tag(
        &self,
        request: Request<RemoveFileTagRequest>,
    ) -> Result<Response<RemoveFileTagResponse>, Status> {

        let body = request.into_inner();

        let file_id: String = body.file_id;
        let tag_name: String = body.tag_name;
        if file_id.trim().is_empty() {
            warn!("[{}] - File id is empty", function_name!());
            return Err(Status::invalid_argument("File id cannot be empty"));
        }
        if tag_name.trim().is_empty() {
            warn!("[{}] - Tag name is empty", function_name!());
            return Err(Status::invalid_argument("Tag name cannot be empty"));
        }

        match super::tag::remove_tag_from_file(&file_id, &tag_name) {
            Ok(_) => {
                info!("[{}] - File tag removed ({}, {})", function_name!(), file_id, tag_name);
                let response = RemoveFileTagResponse {
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't remove file tag, {}", function_name!(), e);
                return Err(Status::internal("Can't remove file tag"));
            },
        }
    }

    #[named]
    async fn get_files_tagged_with(
        &self,
        request: Request<GetFilesTaggedWithRequest>,
    ) -> Result<Response<GetFilesTaggedWithResponse>, Status> {

        let body = request.into_inner();
        let tag_name: String = body.tag_name;
        if tag_name.trim().is_empty() {
            warn!("[{}] - Tag name is empty", function_name!());
            return Err(Status::invalid_argument("Tag name cannot be empty"));
        }

        match super::tag::get_files_tagged_with(&tag_name) {
            Ok(list) => {
                info!("[{}] - Got files tagged with {}", function_name!(), tag_name);
                let mut files: Vec<super::storefile::v1::File> = Vec::new();

                for file in list {
                    files.push(super::storefile::v1::File {
                        id: file.id,
                        name: file.name,
                        identifier: file.identifier,
                        size: file.size,
                        mime_type: file.mime_type,
                        created_at: file.created_at.to_string(),
                        updated_at: file.updated_at.to_string(),
                    });
                }

                let response = GetFilesTaggedWithResponse {
                    files,
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                error!("[{}] -- Can't get files tagged with {}, {}", function_name!(), tag_name, e);
                return Err(Status::internal("Can't get files tagged with"));
            },
        }
    }
}
