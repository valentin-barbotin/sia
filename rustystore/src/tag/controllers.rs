use serde::{Deserialize, Serialize};

use log::{error, warn, info, debug, trace, LevelFilter};
use function_name::named;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use utoipa::{IntoParams, ToSchema};

use crate::file::controllers::FileQuery;

use super::tag;

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreateTag {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct TagQuery {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct TagFileQuery {
    pub tag_name: String,
    pub file_id: String,
}

#[utoipa::path(
    put,
    path = "/create",
    tag = "Tag",
    responses(
        (status = 200, description = "Tag created"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn create_tag(_req: HttpRequest, params: web::Json<CreateTag>) -> HttpResponse {
    if params.name.is_empty() {
        return HttpResponse::UnprocessableEntity().body("Name cannot be empty");
    }

    match tag::create_tag(&params.name) {
        Ok(res) => {
            info!("[{}] - Tag inserted ({})", function_name!(), &params.name);
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            if e.to_string().contains("duplicate key value") {
                return HttpResponse::UnprocessableEntity().body("Tag already exists");
            }
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/remove",
    tag = "Tag",
    responses(
        (status = 200, description = "Tag removed"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn remove_tag(_req: HttpRequest, params: web::Query<TagQuery>) -> HttpResponse {
    if params.name.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Name cannot be empty");
    }

    match tag::remove_tag(&params.name) {
        Ok(_) => {
            info!("[{}] - Tag removed ({})", function_name!(), &params.name);
            HttpResponse::Ok().body(())
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[utoipa::path(
    put,
    path = "/link",
    tag = "Tag",
    responses(
        (status = 200, description = "Tag linked"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn link_file_with_tag(_req: HttpRequest, params: web::Json<TagFileQuery>) -> HttpResponse {
    if params.file_id.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("File id cannot be empty");
    }
    if params.tag_name.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Name cannot be empty");
    }

    match tag::link_file_with_tag(&params.file_id, &params.tag_name) {
        Ok(_) => {
            info!("[{}] - Tag linked ({} -> {})", function_name!(), &params.file_id, &params.tag_name);
            HttpResponse::Ok().body(())
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[utoipa::path(
    get,
    path = "/tags",
    tag = "Tag",
    responses(
        (status = 200, description = "Tags retrieved"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn get_tags(_req: HttpRequest) -> HttpResponse {
    match tag::get_tags() {
        Ok(tags) => {
            info!("[{}] - Tags retrieved", function_name!());
            HttpResponse::Ok().json(tags)
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[utoipa::path(
    get,
    path = "/tags_file",
    tag = "Tag",
    responses(
        (status = 200, description = "Tags retrieved"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn get_tags_of_file(_req: HttpRequest, params: web::Query<FileQuery>) -> HttpResponse {
    if params.id.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Id cannot be empty");
    }

    match tag::get_tags_of_file(&params.id) {
        Ok(tags) => {
            info!("[{}] - Tags retrieved ({:?})", function_name!(), tags);
            HttpResponse::Ok().json(tags)
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/tags_file",
    tag = "Tag",
    responses(
        (status = 200, description = "Tag removed"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn remove_tag_from_file(_req: HttpRequest, params: web::Query<TagFileQuery>) -> HttpResponse {
    if params.file_id.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Id cannot be negative");
    }
    if params.tag_name.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Name cannot be empty");
    }

    match tag::remove_tag_from_file(&params.file_id, &params.tag_name) {
        Ok(_) => {
            info!("[{}] - Tag removed ({} -> {})", function_name!(), &params.tag_name, &params.file_id);
            HttpResponse::Ok().body(())
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[utoipa::path(
    get,
    path = "/get",
    tag = "Tag",
    responses(
        (status = 200, description = "Tags retrieved"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn get_files_tagged_with(_req: HttpRequest, params: web::Query<TagQuery>) -> HttpResponse {
    if params.name.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Name cannot be empty");
    }

    match tag::get_files_tagged_with(&params.name) {
        Ok(tags) => {
            info!("[{}] - Tags retrieved ({:?})", function_name!(), tags);
            HttpResponse::Ok().json(tags)
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
