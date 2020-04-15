use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use chrono::prelude::Utc;
use chrono::SecondsFormat;
use json::JsonValue;
use std::env;

async fn health(request: HttpRequest) -> Result<HttpResponse, Error> {
    let headers = request.headers();
    let now = Utc::now();

    let response_json: JsonValue = json::object! {
        "health" => "Ok",
        "agent" => format!("{}", match headers.get("user-agent") {
            None => "",
            Some(x) => x.to_str().unwrap(),
        }),
        "created_at" => now.to_rfc3339_opts(SecondsFormat::Millis, false),
        "version" => env!("CARGO_PKG_VERSION"),
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response_json.dump()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/health/").route(web::get().to(health)))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header, test, web, App};

    #[actix_rt::test]
    async fn test_api_health_ok() {
        let mut app = test::init_service(App::new().route("/health/", web::get().to(health))).await;
        let req = test::TestRequest::with_header(header::CONTENT_TYPE, "aplication/json")
            .uri("/health/")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
