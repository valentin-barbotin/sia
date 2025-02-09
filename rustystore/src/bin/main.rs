use std::{io::{Write}, net::{SocketAddrV4, Ipv4Addr, SocketAddr}, thread};
use actix_cors::Cors;
use actix_web::{HttpServer, web, error::{JsonPayloadError, self}, http, App, middleware};
use log::{error, warn, info, debug, trace, LevelFilter};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use tonic::{transport::Server, Request, Status};
use tonic_reflection::server::Builder;

use dotenv::dotenv;

extern crate rustystore;

use rustystore::{
    health,
    local_env,
    colors,
    database,
    tag::{
        self,
        grpc::{
            TagSvc,
            tag::tag_service_server::{
                TagService,
                TagServiceServer
            }
        },
    },
    file::{
        self,
        grpc::{
            StorefileSvc,
            storefile::storefile_service_server::{
                StorefileService,
                StorefileServiceServer
            }
        },
    },
};

use local_env::*;
use function_name::named;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::check,
        file::controllers::insert,
        file::controllers::remove,
    ),
)]
struct ApiDoc;

#[tokio::main]
#[named]
async fn main() -> Result<(), std::io::Error> {
    // Load environment variables from .env file, TODO: only in debug mode.
    dotenv().ok();

    local_env::check_vars();

    // Initialize logger
    let level: LevelFilter = match RUST_ENV.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::new()
        .format(|buf, record| {
            let level = match record.level() {
                log::Level::Error => colors::RED,
                log::Level::Warn => colors::YELLOW,
                log::Level::Info => colors::GREEN,
                log::Level::Debug => colors::BLUE,
                log::Level::Trace => colors::CYAN,
            };
            writeln!(buf, "[{}{}{}] - {}", level, record.level(), colors::RESET, record.args())
        })
        .filter(None, level)
        .target(env_logger::Target::Stdout)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let server = HttpServer::new(|| {
        // Extract JSONs from the request body
        let json_cfg = web::JsonConfig::default()
            .limit(2048) // Limit the size of the JSON to 2KB
            .content_type(|mime| {
                // Only accept JSONs
                mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON
            })
            .error_handler(|err, _req| {
                error!("Error: {}", err);
                match err {
                    JsonPayloadError::ContentType => {
                        error::ErrorBadRequest("Invalid content type")
                    },
                    JsonPayloadError::Deserialize(_err) => {
                        error::ErrorBadRequest("Invalid body")
                    },
                    _ => {
                        error!("Unknown error");
                        error::ErrorBadRequest("Unknown error")
                    }
                }
            });

        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "DELETE"])
            .allow_any_origin()
            .send_wildcard()
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(json_cfg)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .route("/health", web::get().to(health::check))
            .service(
                SwaggerUi::new("/documentation/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/v1/file")
                        .service(
                            web::resource("/get")
                                .route(web::get().to(file::controllers::get))
                        )
                        .service(
                            web::resource("/list")
                                .route(web::get().to(file::controllers::list))
                        )
                        .service(
                            web::resource("/insert")
                                .route(web::put().to(file::controllers::insert))
                            
                        )
                        .service(
                            web::resource("/remove")
                                .route(web::delete().to(file::controllers::remove))
                        )
                    )
                    .service(
                        web::scope("/v1/tag")
                        .service(
                            web::resource("/create")
                                .route(web::put().to(tag::controllers::create_tag))
                        )
                        .service(
                            web::resource("/remove")
                                .route(web::delete().to(tag::controllers::remove_tag))
                        )
                        .service(
                            web::resource("/link")
                                .route(web::put().to(tag::controllers::link_file_with_tag))
                        )
                        .service(
                            web::resource("/tags")
                                .route(web::get().to(tag::controllers::get_tags))
                        )
                        .service(
                            web::resource("/tags_file")
                                .route(web::get().to(tag::controllers::get_tags_of_file))
                                .route(web::delete().to(tag::controllers::remove_tag_from_file))
                        )
                        .service(
                            web::resource("/get")
                                .route(web::get().to(tag::controllers::get_files_tagged_with))
                        )
                    )
            )
    });

    info!("Starting server on port {}", *PORT);

    let socket = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), *PORT);

    let grpc = thread::spawn(move || {
        grpc_main().unwrap();
    });

    match server.bind(socket) {
        Ok(server) => {
            server.run().await
        },
        Err(e) => {
            error!("Error while binding to socket: {}", e);
            return Err(e);
        }
    }
}



#[tokio::main]
async fn grpc_main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("0.0.0.0:{}", *PORT_GRPC).parse().unwrap();

    let tag_svc = TagSvc::default();
    let storefile_svc = StorefileSvc::default();

    let tag_service = TagServiceServer::with_interceptor(tag_svc, check_auth);
    let storefile_service = StorefileServiceServer::with_interceptor(storefile_svc, check_auth);

    let reflection = Builder::configure()
        .register_encoded_file_descriptor_set(tag::grpc::tag::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    info!("[{}] -- GRPC Server started", "Main");

    Server::builder()
        .add_service(tag_service)
        .add_service(storefile_service)
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}

/*
    Interceptor to check the authorization token
    Should not start with "Bearer "
*/
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    return Ok(req);
    // let auth_metadata = req.metadata().get("x-apikey");
    // if auth_metadata.is_none() {
    //     warn!("[{}] -- No x-apikey metadata found", "Main");
    //     return Err(Status::unauthenticated("No x-apikey metadata found".to_string()));
    // }

    // match req.metadata().get("x-apikey") {
    //     Some(t) => {
    //         let auth_token = t.to_str().unwrap();

    //         if auth_token.is_empty() {
    //             warn!("[{}] -- No valid auth token", "Main");
    //             return Err(Status::unauthenticated("No valid auth token"));
    //         }

    //         if auth_token != *APIKEY {
    //             warn!("[{}] -- Invalid auth token", "Main");
    //             return Err(Status::unauthenticated("Invalid auth token"));
    //         }

    //         info!("[{}] -- Valid auth token", "Main");

    //         Ok(req)
    //     },
    //     _ => {
    //         warn!("[{}] -- No valid auth token", "Main");
    //         Err(Status::unauthenticated("No valid auth token"))
    //     },
    // }
}
