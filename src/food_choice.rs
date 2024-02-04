#[derive(sqlx::Type, Debug, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "affordability", rename_all = "lowercase")]
pub enum Affordability {
    Low,
    Medium,
    High,
}
#[derive(sqlx::Type, Debug, serde::Serialize, serde::Deserialize, Copy, Clone)]
#[sqlx(type_name = "place", rename_all = "lowercase")]
pub enum Place {
    Home,
    Restaurant,
    Takeout,
}
#[derive(
    Debug, sqlx::FromRow, sqlx::Encode, sqlx::Decode, serde::Serialize, serde::Deserialize,
)]
pub struct FoodChoiceRequest {
    pub(crate) name: String,
    // Cheap, Expensive, or Moderate
    pub(crate) price: Affordability,
    pub(crate) effort: Affordability,
    pub(crate) tag: Place,
}
#[derive(
    Debug, sqlx::FromRow, sqlx::Encode, sqlx::Decode, serde::Serialize, serde::Deserialize,
)]
pub struct FoodChoice {
    pub(crate) name: String,
    // Cheap, Expensive, or Moderate
    pub(crate) price: Affordability,
    pub(crate) effort: Affordability,
    pub(crate) tag: Place,
    pub(crate) owner: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Calendar {
    pub(crate) dates: Vec<String>,
    pub(crate) food_choices: Vec<FoodChoice>,
}
