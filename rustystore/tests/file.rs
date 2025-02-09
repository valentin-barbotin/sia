#[cfg(test)]
mod tests {
    extern crate rustystore;

    use std::fmt::format;

    use chrono::Utc;
    use dotenv::dotenv;

    use lazy_static::lazy_static;
    use rustystore::{
        health::{
            check
        },
        file
    };

    use actix_web::{
        get,
        post,
        web::{
            self,
            Json,
            JsonConfig,
        },
        App,
        http::header::ContentType,
        test,
        HttpResponse,
        HttpServer,
        Responder,
        guard,
        middleware,
        error::{
            self,
            JsonPayloadError
        },
        http::header
    };
    use rstest::*;
    use uuid::Uuid;

    use rustystore::local_env::{
        self, *
    };

    macro_rules! get_json_config {
        () => {
            web::JsonConfig::default()
                .limit(2048) // Limit the size of the JSON to 2KB
                .content_type(|mime| {
                    // Only accept JSONs
                    mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON
            })
        };
    }

    /*
        Simple health check
     */
    #[actix_web::test]
    async fn healthcheck() {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/health")
                    .route(web::get().to(check))
            );

        let app = test::init_service(webapp).await;
        let req = test::TestRequest::get().uri("/health").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[rstest]
    #[case::empty_identifier("")]
    #[case::identifier("1234_5678")]
    #[actix_web::test]
    async fn try_to_get_file(#[case] identifier: &str) {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/api/v1/get")
                    .route(web::get().to(file::controllers::get))
            );

        let app = test::init_service(webapp).await;

        let uri = format!("/api/v1/get?id={}", identifier);

        let req = test::TestRequest::get()
            .uri(&uri)
            .to_request();

        let resp = test::call_service(&app, req).await;

        if identifier.trim().is_empty() {
            assert!(resp.status().is_client_error());
        } else {
            assert!(resp.status().is_success());
        }
    }

    #[rstest]
    #[actix_web::test]
    async fn try_to_list_files() {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/api/v1/list")
                    .route(web::get().to(file::controllers::list))
            );

        let app = test::init_service(webapp).await;

        let uri = format!("/api/v1/list");

        let req = test::TestRequest::get()
            .uri(&uri)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[rstest]
    #[case::empty_identifier("")]
    #[case::identifier("1234_5678")]
    #[actix_web::test]
    async fn try_to_remove_file(#[case] identifier: &str) {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/api/v1/delete")
                    .route(web::get().to(file::controllers::remove))
            );

        let app = test::init_service(webapp).await;

        let uri = format!("/api/v1/delete?id={}", identifier);

        let req = test::TestRequest::delete()
            .uri(&uri)
            .to_request();

        let resp = test::call_service(&app, req).await;

        if identifier.trim().is_empty() {
            assert!(resp.status().is_client_error());
        } else {
            assert!(resp.status().is_success());
        }
    }

    #[rstest]
    #[case::valid_with_empty_tags("file1.txt", "abcd", 5, "text/plain", vec![])]
    #[case::negative_size("file1.txt", "abcd", -10, "text/plain", vec![])]
    #[case::empty_param2("", "abcd", 5, "text/plain", vec![])]
    #[case::empty_param3("file1.txt", "", 5, "text/plain", vec![])]
    #[case::empty_param4("file1.txt", "abcd", 5, "", vec![])]
    #[actix_web::test]
    async fn try_to_insert_file(#[case] name: String, #[case] identifier: String, #[case] size: i64, #[case] mime_type: String, #[case] tags: Vec<String>) {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/api/v1/insert")
                    .route(web::get().to(file::controllers::insert))
            );

        let app = test::init_service(webapp).await;

        let uri = format!("/api/v1/insert");

        let data = file::controllers::FileInfo {
            name: name.to_owned(),
            identifier: identifier.to_owned(),
            size,
            mime_type: mime_type.to_owned(),
            tags
        };

        let req = test::TestRequest::delete()
            .uri(&uri)
            .set_json(data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        match resp.status() {
            actix_web::http::StatusCode::OK => {
                assert!(!name.trim().is_empty());
                assert!(!identifier.trim().is_empty());
                assert!(!mime_type.trim().is_empty());
                assert!(size > 0);
                assert!(resp.status().is_success());
            },
            _ => {
                assert!(resp.status().is_client_error());
            }
        }
    }
}