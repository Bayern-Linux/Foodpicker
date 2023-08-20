use google_maps::directions::Location;
use google_maps::distance_matrix::TravelMode;
use google_maps::prelude::Decimal;
use google_maps::prelude::Place as MapsPlace;
use tracing::info;

pub async fn find_food_nearby() -> Result<Vec<MapsPlace>, google_maps::error::Error> {
    let maps_client = google_maps::GoogleMapsClient::new(
        std::env::var("MAPS_API_KEY")
            .expect("MAPS_API_KEY not set")
            .as_str(),
    );
    let restaurants = maps_client
        .text_search("restaurants".to_string(), 5000)
        .execute()
        .await
        .unwrap();
    let filtered_restaurants = restaurants
        .results
        .into_iter()
        .filter(|x| {
            x.rating
                .is_some_and(|x| x > Decimal::from_f32_retain(3.5).unwrap())
        })
        .collect();
    Ok(filtered_restaurants)
}
