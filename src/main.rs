use crate::food_choice::{Affordability, Place};
use actix_web::{App, HttpServer};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::Mutex;

mod food_choice;
mod maps;
mod queries;
mod webpage;

struct AppState {
    pool: Mutex<Pool<Postgres>>,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    let pool = sqlx::postgres::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    HttpServer::new(move || {
        let app_state = AppState {
            pool: Mutex::new(pool.clone()),
        };
        App::new().app_data(app_state).service(webpage::index)
    })
    .bind("127.60.20.1:7373")?
    .run()
    .await?;
    let food_choice = food_choice::FoodChoice {
        name: "Pizza".to_string(),
        price: Affordability::Low,
        tag: Place::Home,
        effort: Affordability::Low,
    };
    Ok(())
}
