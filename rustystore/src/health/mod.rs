use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};


#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is running"),
    )
)]
pub async fn check(_req: HttpRequest) -> HttpResponse {
    // info!("[{}] -- Check", "Health");

    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test_check_ok() {

        let req = test::TestRequest::get()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = check(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
