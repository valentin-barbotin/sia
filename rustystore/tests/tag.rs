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
        tag, getConn
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

    fn seed_database_tag() {
        use rustystore::schema::store::tag::dsl::*;
        use diesel::prelude::*;
        dotenv().ok();
        local_env::check_vars();
        let conn = getConn!();

        // clean database
        diesel::delete(tag).execute(conn).unwrap();

        // insert data
        diesel::insert_into(tag).values((
            name.eq("tag1")
        )).execute(conn).unwrap();

        diesel::insert_into(tag).values((
            name.eq("tag_to_remove")
        )).execute(conn).unwrap();
    }

    fn seed_database_file() {
        use rustystore::schema::store::file::dsl::*;
        use diesel::prelude::*;
        dotenv().ok();
        local_env::check_vars();
        let conn = getConn!();

        // clean database
        diesel::delete(file).execute(conn).unwrap();

        // insert data
        diesel::insert_into(file).values((
            id.eq(1),
            name.eq("test"),
            identifier.eq("test"),
            size.eq(1),
        )).execute(conn).unwrap();
    }

    fn seed_database_file_tag() {
        use rustystore::schema::store::file_tag::dsl::*;
        use diesel::prelude::*;
        dotenv().ok();
        local_env::check_vars();
        let conn = getConn!();

        // clean database
        diesel::delete(file_tag).execute(conn).unwrap();

        // insert data
        diesel::insert_into(file_tag).values((
            tag_id.eq(1),
            file_id.eq(1)
        )).execute(conn).unwrap();

        diesel::insert_into(file_tag).values((
            tag_id.eq(2),
            file_id.eq(1)
        )).execute(conn).unwrap();
    }

    #[fixture]
    #[once]
    fn seed_database() {
        seed_database_tag();
        seed_database_file();
        seed_database_file_tag();
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

    /*
        Test the tag::create function
     */
    #[actix_web::test]
    async fn create_tag() {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/create")
                    .route(web::put().to(tag::controllers::create_tag))
            );

        let app = test::init_service(webapp).await;
        let req = test::TestRequest::put()
            .uri("/create")
            .set_json(&tag::controllers::CreateTag {
                name: "test".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    /*
        Test the tag::remove function
     */
    #[actix_web::test]
    async fn remove_tag() {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/remove")
                    .route(web::delete().to(tag::controllers::remove_tag))
            );

        let app = test::init_service(webapp).await;
        let req = test::TestRequest::delete()
            .param("name", "tag_to_remove")
            .uri("/remove")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    /*
        Test the tag::link function
     */
    #[actix_web::test]
    async fn link_tag() {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/link")
                    .route(web::put().to(tag::controllers::link_file_with_tag))
            );

        let app = test::init_service(webapp).await;
        let req = test::TestRequest::put()
            .uri("/link")
            .set_json(&tag::controllers::TagFileQuery {
                tag_name: "tag1".to_string(),
                file_id: Uuid::new_v4().to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    /*
        Test the tag::tags function
     */
    #[actix_web::test]
    async fn tags_tag() {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/tags")
                    .route(web::get().to(tag::controllers::get_tags))
            );

        let app = test::init_service(webapp).await;
        let req = test::TestRequest::get()
            .uri("/tags")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    /*
        Test the tag::tags_file (get) function
     */
    #[actix_web::test]
    async fn tags_file_tag() {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/tags_file")
                    .route(web::get().to(tag::controllers::get_tags_of_file))
            );
        
        let app = test::init_service(webapp).await;
        let req = test::TestRequest::get()
            .param("name", "tag1")
            .uri("/tags_file")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    /*
        Test the tag::tags_file (delete) function
     */
    #[actix_web::test]
    async fn tags_file_delete_tag() {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/tags_file")
                    .route(web::delete().to(tag::controllers::remove_tag_from_file))
            );
        
        let app = test::init_service(webapp).await;
        let req = test::TestRequest::delete()
            .param("file_id", Uuid::new_v4().to_string())
            .param("name", "tag1")
            .uri("/tags_file")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    /*
        Test the tag::get function
     */
    #[actix_web::test]
    async fn get_files_tagged_with() {
        dotenv().ok();
        local_env::check_vars();

        let webapp = App::new()
            .app_data(get_json_config!())
            .service(
                web::resource("/get")
                    .route(web::get().to(tag::controllers::get_files_tagged_with))
            );

        let app = test::init_service(webapp).await;
        let req = test::TestRequest::get()
            .param("name", "tag1")
            .uri("/get")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}