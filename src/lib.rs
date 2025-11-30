use std::{fs};
use serde::{Deserialize, Serialize};
use reqwest::get;

//----------------------------Async-weather-API-calls-done-here!------------------------------------
//In order for system to work, create a file called "apikey.txt" and locate it in the src folder.
//The txt file should also include an API key associated with you, handed out by the danish DMI weather stations website
pub async fn getLowestTemp() -> (f32, [f32; 2]){
    let apiKey = fs::read_to_string("src/apikey.txt").unwrap();
    let request = format!("https://dmigw.govcloud.dk/v2/metObs/collections/observation/items?limit=200&bbox=7,54,16,58&parameterId=temp_min_past1h&api-key={}", apiKey.trim());
    let resp = get(request)
        .await.expect("should respond with JSON")
        .json::<DataInput>()
        .await;
    resp.unwrap().getMinTemp()
}
impl DataInput {
    fn getMinTemp(&self) -> (f32, [f32; 2]) {
        let mut minTemp: f32 = f32::INFINITY;
        let mut latitude: f32 = 0.0;
        let mut longitude: f32 = 0.0;

        for feature in &self.features {
            if (feature.properties.value < minTemp){
                minTemp = feature.properties.value;
                latitude = feature.geometry.coordinates[0];
                longitude = feature.geometry.coordinates[1];
            }
        }
        (minTemp, [longitude, latitude])
    }
}

#[derive(Deserialize, Debug, Clone)]
struct DataInput {
    features: Vec<Feature>,
}
#[derive(Deserialize, Debug, Clone)]
struct Feature {
    geometry: Geometry,
    properties: Properties,
}

#[derive(Deserialize, Debug, Clone, )]
struct Geometry {
    coordinates: [f32; 2]
}

#[derive(Deserialize, Debug, Clone)]
struct Properties{
    value: f32
}
