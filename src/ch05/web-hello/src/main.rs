use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use std::fs;

const SERVER_ADDR: &str = "127.0.0.1:8880";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[Serever]");
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(SERVER_ADDR)?
        .run()
        .await
}

// //특정 파일의 app/build/index.html 불러오는 것으로 개발해줘
async fn index(_: HttpRequest) -> Result<HttpResponse, Error> {
    let index_html = fs::read_to_string("../app/build/index.html").unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index_html))
}
