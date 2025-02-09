use std::{println, fs::File, io::{Read, Write}, path::Path, format};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};

use serde::Deserialize;

use actix_multipart::{
    form::{
        json, tempfile::{TempFile, TempFileConfig}, text::Text, MultipartForm
    },
    Multipart,
};
use log::{error, warn, info, debug, trace, LevelFilter};
use function_name::named;
use actix_web::{body::BoxBody, web, HttpRequest, HttpResponse, Result};
use actix_files::NamedFile;
use utoipa::IntoParams;


#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    file: TempFile,
    secretKey: Text<String>,
    // json: json::Json<Metadata>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct FileQuery {
    id: String,
    secretKey: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct FileQueryWithoutKey {
    id: String,
}

use super::file::{
    check_if_exists,
    download,
    delete,
    upload as upload_file,
    generate_uuid
};

#[utoipa::path(
    put,
    path = "/upload",
    tag = "File",
    responses(
        (status = 200, description = "File uploaded"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn upload(req: HttpRequest, MultipartForm(form): MultipartForm<UploadForm>) -> HttpResponse {
    let uuid = generate_uuid().await;

    let file: TempFile = form.file;
    let file_name = file.file_name.unwrap();
    let mut file: File = file.file.into_file();

    let secret_key = &form.secretKey.0;
    let decoded_key = match hex::decode(secret_key) {
        Ok(key) => key,
        Err(e) => {
            error!("[{}] -- {}", function_name!(), e.to_string());
            return HttpResponse::InternalServerError().body(());
        }
    };

    let key_array: [u8; 32] = match decoded_key.try_into() {
        Ok(key) => key,
        Err(e) => {
            return HttpResponse::InternalServerError().body(());
        }
    };

    let key = Key::<Aes256Gcm>::from_slice(&key_array);

    let cipher = Aes256Gcm::new(&key);

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    // info!("[{}] -- Nonce: {:?}", function_name!(), nonce);

    info!("[{}] -- Start Encrypted data", function_name!());
    let encrypted_data = cipher.encrypt(&nonce, buffer.as_slice()).unwrap();
    info!("[{}] -- End Encrypted data", function_name!());
    // info!("[{}] -- 1Buffer: {:?}", function_name!(), encrypted_data);
    let mut encrypted_message = Vec::new();
    encrypted_message.extend_from_slice(&nonce);
    // info!("[{}] -- 2Buffer: {:?}", function_name!(), encrypted_message);
    encrypted_message.extend_from_slice(&encrypted_data);
    // info!("[{}] -- 3Buffer: {:?}", function_name!(), encrypted_message);

    match upload_file(encrypted_message, &uuid).await {
        Ok(_) => {
            info!("[{}] -- File {} uploaded as {}", function_name!(), file_name, uuid);
        },
        Err(e) => {
            error!("[{}] -- {}", function_name!(), e.to_string());
            return HttpResponse::InternalServerError().body(());
        }
    };

    HttpResponse::Ok().body(uuid)
}

#[utoipa::path(
    get,
    path = "/download",
    params(FileQuery),
    tag = "File",
    responses(
        (status = 200, description = "File downloaded"),
        (status = 204, description = "File does not exist"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn download_file(req: HttpRequest, params: web::Query<FileQuery>) -> HttpResponse {
    if params.id.is_empty() {
        return HttpResponse::BadRequest().body("File id is empty");
    }

    if params.secretKey.is_empty() {
        return HttpResponse::BadRequest().body("Secret key is empty");
    }

    let secret_key = &params.secretKey;
    let decoded_key = match hex::decode(secret_key) {
        Ok(key) => key,
        Err(e) => {
            error!("[{}] -- {}", function_name!(), e.to_string());
            return HttpResponse::InternalServerError().body(());
        }
    };

    let key_array: [u8; 32] = match decoded_key.try_into() {
        Ok(key) => key,
        Err(e) => {
            return HttpResponse::InternalServerError().body(());
        }
    };

    let key = Key::<Aes256Gcm>::from_slice(&key_array);

    let cipher = Aes256Gcm::new(&key);

    match download(&params.id).await {
        Ok(file) => {
            let bytes = match file.bytes().await {
                Ok(bytes) => {
                    let stored_nonce = &bytes[..12]; // 96 bits = 12 octets
                    let stored_ciphertext = &bytes[12..];

                    let nonce_array: [u8; 12] = match stored_nonce.try_into() {
                        Ok(nonce) => nonce,
                        Err(e) => {
                            error!("1[{}] -- {}", function_name!(), e.to_string());
                            return HttpResponse::InternalServerError().body(());
                        }
                    };

                    let nonce = Nonce::from_slice(&nonce_array);
                    // info!("[{}] -- Nonce: {:?}", function_name!(), nonce);
                    // info!("[{}] -- Stored ciphertext: {:?}", function_name!(), stored_ciphertext);

                    match cipher.decrypt(nonce, stored_ciphertext) {
                        Ok(data) => data,
                        Err(e) => {
                            error!("[{}] -- {}", function_name!(), e.to_string());
                            return HttpResponse::BadRequest().body("Invalid secret key");
                        }
                    }
                },
                Err(e) => {
                    error!("3[{}] -- {}", function_name!(), e.to_string());
                    return HttpResponse::InternalServerError().body(());
                }
                
            };

            let content_disposition = format!("attachment; filename=\"{}\"", params.id);
            let content_type = "application/octet-stream"; // ou le type de contenu approprié

            return HttpResponse::Ok()
                .content_type(content_type)
                .append_header(("Content-Disposition", content_disposition))
                .body(bytes.to_vec());
        },
        Err(e) => {
            error!("[{}] -- {}", function_name!(), e.to_string());
            return HttpResponse::InternalServerError().body(());
        }
    }
}

#[utoipa::path(
    delete,
    path = "/delete",
    params(FileQuery),
    tag = "File",
    responses(
        (status = 200, description = "File deleted"),
        (status = 404, description = "File does not exist"),
        (status = 500, description = "Internal server error"),
    )
)]
#[named]
pub async fn delete_file(req: HttpRequest, params: web::Query<FileQueryWithoutKey>) -> HttpResponse {
    if params.id.is_empty() {
        return HttpResponse::BadRequest().body("File id is empty");
    }

    match delete(&params.id).await {
        Ok(_) => {
            info!("[{}] -- File {} deleted", function_name!(), &params.id);
            HttpResponse::Ok().body(())
        },
        Err(e) => {
            error!("[{}] -- {}", function_name!(), e.to_string());
            return HttpResponse::InternalServerError().body(());
        }
    }
}