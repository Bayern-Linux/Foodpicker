use crate::food_choice::{Affordability, FoodChoice, Place};
use sqlx::{Pool, Postgres};

// Write food choice to postgres.sql via sqlx
pub async fn write_food_choice_to_db(
    pool: &Pool<Postgres>,
    food_choice: FoodChoice,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO food_choice (name, price, effort, tag) VALUES ($1, $2, $3, $4)",
        food_choice.name,
        food_choice.price as Affordability,
        food_choice.effort as Affordability,
        food_choice.tag as Place,
    )
    .execute(pool)
    .await?;
    Ok(())
}
