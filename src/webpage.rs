use crate::food_choice::FoodChoice;
use crate::queries;
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

#[derive(serde::Deserialize, Debug)]
pub struct Payload {
    value: String,
}

#[post("/send-food-choice")]
pub(crate) async fn send_food_choice(food_choice: web::Json<FoodChoice>) -> impl Responder {
    let pool = sqlx::postgres::PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let food_choice = food_choice.into_inner();
    let food_choice = FoodChoice {
        name: food_choice.name,
        price: food_choice.price,
        effort: food_choice.effort,
        tag: food_choice.tag,
    };
    let _ = queries::write_food_choice_to_db(pool, food_choice).await;
    HttpResponse::Ok()
}
