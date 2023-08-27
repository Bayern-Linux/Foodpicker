use crate::food_choice::{Affordability, FoodChoice, Place};
use sqlx::{Pool, Postgres};

// Write food choice to postgres.sql via sqlx
pub async fn write_food_choice_to_db(
    pool: &Pool<Postgres>,
    food_choice: FoodChoice,
) -> Result<(), sqlx::Error> {
    // TODO: Check if food_choice already exists in postgres.sql if so, update it
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
// Delete food choice from postgres.sql via sqlx
pub async fn delete_food_choice_from_db(
    pool: &Pool<Postgres>,
    food_choice: FoodChoice,
    food_tag: Place,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM food_choice WHERE name = $1 AND tag = $2",
        food_choice.name,
        food_tag as Place,
    )
    .execute(pool)
    .await?;
    Ok(())
}

// Read food choice from postgres.sql via sqlx
pub async fn read_food_choice_from_db(
    pool: &Pool<Postgres>,
    name: String,
) -> Result<FoodChoice, sqlx::Error> {
    let food_choice = sqlx::query_as!(
        FoodChoice,
        r#"SELECT name, price as "price: _", effort as "effort: _", tag as "tag: _" FROM food_choice WHERE name = $1"#,
        name
    )
    .fetch_one(pool)
    .await?;
    Ok(food_choice)
}
// Read random food choice from postgres.sql via sqlx
pub async fn read_random_food_choice_from_db(
    pool: &Pool<Postgres>,
    amount: i64,
    tag: Place,
) -> Result<Vec<FoodChoice>, sqlx::Error> {
    let food_choices = sqlx::query_as!(
        FoodChoice,
        r#"SELECT name, price as "price: _", effort as "effort: _", tag as "tag: _" FROM food_choice WHERE tag = $1 ORDER BY RANDOM() LIMIT $2"#,
        tag as Place,
        amount
    )
        .fetch_all(pool)
        .await?;
    Ok(food_choices)
}
