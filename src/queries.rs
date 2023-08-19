use sqlx::{Pool, Postgres};



// Write food choice to postgres via sqlx
async fn write_food_choice_to_db(pool: &mut Pool<Postgres>, food_choice: FoodChoice) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO food_choice (name, price, afford, tag) VALUES ($1, $2, $3, $4)",
        food_choice.name,
        food_choice.price,
        food_choice.afford,
        food_choice.tag,
    )
        .execute(&mut pool)
        .await?;
    Ok(())
}