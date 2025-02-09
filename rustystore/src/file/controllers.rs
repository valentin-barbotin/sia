use std::{println, fs::File, io::Read};
use serde::{Deserialize, Serialize};

use log::{error, warn, info, debug, trace, LevelFilter};
use function_name::named;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use utoipa::{IntoParams, ToSchema};

use super::{
    file
};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct FileQuery {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct FileInfo {
    pub name: String,
    pub identifier: String,
    pub size: i64,
    pub mime_type: String,
    pub tags: Vec<String>
}

#[utoipa::path(
    put,
    path = "/insert",
    tag = "File",
    responses(
        (status = 200, description = "File inserted"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn insert(_req: HttpRequest, params: web::Json<FileInfo>) -> HttpResponse {
    if params.name.is_empty() {
        return HttpResponse::UnprocessableEntity().body("Name cannot be empty");
    }
    if params.identifier.is_empty() {
        return HttpResponse::UnprocessableEntity().body("Identifier cannot be empty");
    }
    if params.size < 0 {
        return HttpResponse::UnprocessableEntity().body("Size cannot be negative");
    }
    if params.mime_type.is_empty() {
        return HttpResponse::UnprocessableEntity().body("Mime type cannot be empty");
    }


    match file::insert(&params) {
        Ok(_) => {
            info!("[{}] - File inserted ({:?})", function_name!(), params);
            HttpResponse::Ok().body(())
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            if e.to_string().contains("duplicate key value") {
                return HttpResponse::UnprocessableEntity().body("File already exists");
            }
            HttpResponse::InternalServerError().body(())
        },
    }
}

#[utoipa::path(
    delete,
    path = "/delete",
    tag = "File",
    responses(
        (status = 200, description = "File removed"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn remove(_req: HttpRequest, params: web::Query<FileQuery>) -> HttpResponse {
    if params.id.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Identifier cannot be empty");
    }

    match file::remove(&params.id) {
        Ok(_) => {
            info!("[{}] - File removed ({})", function_name!(), &params.id);
            HttpResponse::Ok().body(())
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            HttpResponse::InternalServerError().body(())
        },
    }
}

#[named]
pub async fn list(_req: HttpRequest) -> HttpResponse {
    match file::get_all() {
        Ok(files) => {
            info!("[{}] - Files listed", function_name!());
            HttpResponse::Ok().json(files)
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            HttpResponse::InternalServerError().body(())
        },
    }
}

#[utoipa::path(
    get,
    path = "/get",
    tag = "File",
    responses(
        (status = 200, description = "File retrieved"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn get(_req: HttpRequest, params: web::Query<FileQuery>) -> HttpResponse {
    if params.id.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().body("Identifier cannot be empty");
    }

    match file::get(&params.id) {
        Ok(res) => {
            info!("[{}] - File retrieved ({})", function_name!(), &params.id);
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            error!("[{}] - {}", function_name!(), e);
            HttpResponse::InternalServerError().body(())
        },
    }
}