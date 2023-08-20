use crate::food_choice::FoodChoice;
use crate::{queries, AppState};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use liquid::{object, Template};
use std::fs::read_to_string;
use std::path::Path;
use tracing::info;

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

#[post("/send-food-choice")]
pub(crate) async fn send_food_choice(
    food_choice: web::Json<FoodChoice>,
    data: web::Data<AppState>,
) -> impl Responder {
    // HACK: This should be a pool that is passed in from main.rs
    let pool = data.pool.lock().await.clone();
    let food_choice = food_choice.into_inner();
    let food_choice = FoodChoice {
        name: food_choice.name,
        price: food_choice.price,
        effort: food_choice.effort,
        tag: food_choice.tag,
    };
    let _ = queries::write_food_choice_to_db(&pool, food_choice).await;
    HttpResponse::Ok()
}

#[get("/get-food-choice")]
pub(crate) async fn get_food_choice(data: web::Data<AppState>) -> impl Responder {
    // TODO: Get a tag so it can only return a food choice that matches that tag
    info!("Getting food choice");
    let pool = data.pool.lock().await.clone();
    let food_choice = queries::read_random_food_choice_from_db(&pool)
        .await
        .unwrap();
    HttpResponse::Ok().json(food_choice)
}
