use crate::food_choice::{Affordability, Place};

mod food_choice;
mod queries;
mod maps;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    let mut pool = sqlx::postgres::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let food_choice = food_choice::FoodChoice {
        name: "Pizza".to_string(),
        price: Affordability::Low,
        tag: Place::Home,
        effort: Affordability::Low,
    };
    queries::write_food_choice_to_db(&pool, food_choice).await?;
    maps::find_food_nearby().await;
    Ok(())
}
