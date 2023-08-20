use crate::food_choice::{Affordability, Place};
use actix_web::{App, HttpServer};

mod food_choice;
mod maps;
mod queries;
mod webpage;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    HttpServer::new(|| App::new().service(webpage::index))
        .bind("127.60.20.1:7373")?
        .run()
        .await?;
    let mut pool = sqlx::postgres::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let food_choice = food_choice::FoodChoice {
        name: "Pizza".to_string(),
        price: Affordability::Low,
        tag: Place::Home,
        effort: Affordability::Low,
    };
    Ok(())
}
