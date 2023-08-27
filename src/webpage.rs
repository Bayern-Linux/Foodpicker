use crate::food_choice::{Calendar, FoodChoice, Place};
use crate::{maps, queries, AppState};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use google_maps::LatLng;
use liquid::{object, Template};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
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

#[get("/get-food-choice?tag={tag}")]
pub(crate) async fn get_food_choice(data: web::Data<AppState>, tag: web::Query<Place>) -> impl Responder {
    // TODO: Get a tag so it can only return a food choice that matches that tag
    info!("Getting food choice");
    let pool = data.pool.lock().await.clone();
    let food_choice = queries::read_random_food_choice_from_db(&pool, 1, tag)
        .await
        .unwrap();
    HttpResponse::Ok().json(food_choice.first())
}
#[get("/get-food-choice-week")]
pub(crate) async fn get_food_choice_week(data: web::Data<AppState>) -> impl Responder {
    // TODO: Get a tag so it can only return a food choice that matches that tag
    info!("Getting food choice");
    let pool = data.pool.lock().await.clone();
    let food_choice = queries::read_random_food_choice_from_db(&pool, 7)
        .await
        .unwrap();
    HttpResponse::Ok().json(food_choice)
}
#[post("/export-to-calendar")]
pub async fn export_to_calendar(
    calendar_data: web::Json<Calendar>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("{:?}", calendar_data);

    Ok(HttpResponse::Ok().finish())
}

#[get("/restaurants")]
pub(crate) async fn restaurants(data: web::Data<AppState>) -> impl Responder {
    let template = liquid::ParserBuilder::with_stdlib().build().unwrap();
    let template = template
        .parse(read_to_string("src/frontend/maps.liquid").unwrap().as_str())
        .unwrap();
    HttpResponse::Ok().body(template.render(&object!({})).unwrap())
}

#[post("/location")]
pub(crate) async fn restaurants_near_location(location_data: web::Json<LatLng>) -> impl Responder {
    info!("Location data: {:?}", location_data);
    let results = maps::find_food_nearby(location_data.0).await.unwrap();
    let mut rng = thread_rng();
    let choice = results.choose(&mut rng).unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&choice).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    // Use this to new food as Json into the DB
    async fn insert_into_db() {
        let pool = sqlx::postgres::PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        let food_choices: Vec<FoodChoice> = serde_json::from_str(
            r#"
"#,
        )
        .unwrap();
        for food_choice in food_choices {
            let _ = queries::write_food_choice_to_db(&pool, food_choice).await;
        }
    }
}
