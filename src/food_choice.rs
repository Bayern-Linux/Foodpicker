
enum Affordability {
    Low,
    Medium,
    High,
}
enum Place {
    Home,
    Restaurant,
    Takeout,
}
struct FoodChoice {
    name: String,
    // Cheap, Expensive, or Moderate
    price: Affordability,
    effort: Affordability,
    tag: Place,
}

