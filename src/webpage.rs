use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use liquid::{object, Template};
use std::fs::read_to_string;
use std::path::Path;

pub(crate) fn liquid_parse(path: impl AsRef<Path>) -> Template {
    let compiler = liquid::ParserBuilder::with_stdlib()
        .build()
        .expect("Could not build liquid compiler");
    compiler
        .parse(read_to_string(path).unwrap().as_str())
        .unwrap()
}

#[get("/")]
pub(crate) async fn index() -> HttpResponse {
    let body = read_to_string("src/frontend/index.liquid").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
