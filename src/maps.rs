use google_maps::prelude::Decimal;
use google_maps::prelude::Place as MapsPlace;
use google_maps::LatLng;

pub async fn find_food_nearby(
    location: LatLng,
) -> Result<Vec<MapsPlace>, google_maps::error::Error> {
    let maps_client = google_maps::GoogleMapsClient::new(
        std::env::var("MAPS_API_KEY")
            .expect("MAPS_API_KEY not set")
            .as_str(),
    );
    let restaurants = maps_client
        .text_search("restaurants".to_string(), 5000)
        .with_location(location)
        .execute()
        .await
        .unwrap();
    let filtered_restaurants = restaurants
        .results
        .into_iter()
        .filter(|x| {
            x.rating
                .is_some_and(|x| x > Decimal::from_f32_retain(3.5).unwrap())
                && x.opening_hours
                    .clone()
                    .is_some_and(|x| x.open_now.is_some_and(|x| x))
        })
        .collect();
    Ok(filtered_restaurants)
}
