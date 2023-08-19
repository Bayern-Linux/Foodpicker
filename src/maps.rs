use google_maps::directions::Location;
use google_maps::distance_matrix::TravelMode;
use google_maps::prelude::Decimal;

pub async fn find_food_nearby(){
    let maps_client = google_maps::GoogleMapsClient::new(std::env::var("MAPS_API_KEY").expect("MAPS_API_KEY not set").as_str());
    let directions = maps_client.text_search("Pizza".to_string(), 5000).execute().await.unwrap();
    directions.results.iter().filter(|x| x.rating.is_some_and(|x| x > Decimal::from_f32_retain(3.5).unwrap())).for_each(|x| println!("{:?}", x.name));
}