use std::{io::{Write}, net::{SocketAddrV4, Ipv4Addr, SocketAddr}, thread};
use actix_cors::Cors;
use actix_web::{HttpServer, web, error::{JsonPayloadError, self}, http, App, middleware};
use log::{error, warn, info, debug, trace, LevelFilter};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

use dotenv::dotenv;

extern crate rustystorage;

use rustystorage::{
    file,
    health,
    local_env,
    colors,
    // s3::{
    //     client
    // },
    file::{
        // self,
        grpc::{
            FileSvc,
            file::file_service_server::{
                FileService,
                FileServiceServer
            }
        },
        file as file_proto
    },
};

use local_env::*;
use function_name::named;

use tonic::{transport::Server, Request, Status};
use tonic_reflection::server::Builder;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::check,
        file::controllers::upload,
        file::controllers::download_file,
        file::controllers::delete_file,
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


    // match client::create_bucket().await {
    //     Ok(_) => {},
    //     Err(e) => {
    //         if e.to_string().contains("BucketAlreadyOwnedByYou") {
    //             info!("[{}] -- Bucket already exists", function_name!());
    //         } else {
    //             error!("[{}] -- Bucket creation error {}", function_name!(), e);
    //             panic!("{}", e)
    //         }
    //     }
    // }

    let grpc = thread::spawn(move || {
        grpc_main().unwrap();
    });

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
                        web::scope("/v1")
                        .service(
                            web::resource("/upload")
                                .route(web::put().to(file::controllers::upload))
                            
                        )
                        .service(
                            web::resource("/download")
                                .route(web::get().to(file::controllers::download_file))
                        )
                        .service(
                            web::resource("/delete")
                                .route(web::delete().to(file::controllers::delete_file))
                        )
                    )
            )
    });

    info!("Starting server on port {}", *PORT);

    let socket = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), *PORT);

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
#[named]
async fn grpc_main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = format!("0.0.0.0:{}", *PORT_GRPC).parse().unwrap();

    let file_svc = FileSvc::default();

    let file_service = FileServiceServer::with_interceptor(file_svc, check_auth);

    let reflection = Builder::configure()
        .register_encoded_file_descriptor_set(file::grpc::file::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    info!("[{}] -- GRPC Server started - {}", function_name!(), addr.to_string());

    Server::builder()
        .add_service(file_service)
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}

/*
    Interceptor to check the authorization token
    Should not start with "Bearer "
*/
#[named]
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let auth_metadata = req.metadata().get("authorization");
    if auth_metadata.is_none() {
        warn!("[{}] -- No authorization metadata found", function_name!());
        return Err(Status::unauthenticated("No authorization metadata found".to_string()));
    }

    match req.metadata().get("authorization") {
        Some(t) => {
            let auth_token = t.to_str().unwrap();

            if auth_token.is_empty() {
                warn!("[{}] -- No valid auth token", function_name!());
                return Err(Status::unauthenticated("No valid auth token"));
            }

            //TODO token metadata

            info!("[{}] -- Valid auth token", function_name!());

            Ok(req)
        },
        _ => {
            warn!("[{}] -- No valid auth token", function_name!());
            Err(Status::unauthenticated("No valid auth token"))
        },
    }
}
