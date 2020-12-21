#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Forecast {
    #[serde(rename = "DateTime")]
    pub date_time: String,
    #[serde(rename = "EpochDateTime")]
    pub epoch_date_time: i64,
    #[serde(rename = "WeatherIcon")]
    pub weather_icon: i32,
    #[serde(rename = "IconPhrase")]
    pub icon_phrase: String,
    #[serde(rename = "HasPrecipitation")]
    pub has_precipitation: bool,
    #[serde(rename = "PrecipitationType")]
    pub precipitation_type: Option<String>,
    #[serde(rename = "PrecipitationIntensity")]
    pub precipitation_intensity: Option<String>,
    #[serde(rename = "IsDaylight")]
    pub is_daylight: bool,
    #[serde(rename = "Temperature")]
    pub temperature: Temperature,
    #[serde(rename = "PrecipitationProbability")]
    pub precipitation_probability: i32,
    #[serde(rename = "MobileLink")]
    pub mobile_link: String,
    #[serde(rename = "Link")]
    pub link: String,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Temperature {
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Unit")]
    pub unit: String,
    #[serde(rename = "UnitType")]
    pub unit_type: i32,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Item {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    #[serde(rename = "Forecasts")]
    pub forecasts: Vec<Forecast>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

use chrono::prelude::{SecondsFormat, Utc};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
use std::env;

pub async fn handler(event: serde_json::Value, _: lambda::Context) -> Result<(), Error> {
    println!("Event: {:?}", event);
    let location_id = "2171775"; // ice pond
    let api_key = env::var("API_KEY").unwrap();
    let table_name = env::var("TABLE_NAME").unwrap();

    let url = format!("http://dataservice.accuweather.com/forecasts/v1/hourly/12hour/{location_id}?apikey={api_key}", location_id=location_id, api_key=api_key);
    println!("Request start: {}", &url);
    let resp = reqwest::get(&url).await.unwrap();
    println!("Request finished");

    let json = resp.json::<Vec<Forecast>>().await.unwrap();

    let client = DynamoDbClient::new(Region::UsEast1);

    // Format for Dynamo 2018-01-26T18:30:09.453Z
    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let put_item: Item = Item {
        pk: location_id.to_string(),
        sk: format!("CREATED#{}", now),
        forecasts: json,
    };

    println!("Writing to DynamoDB");
    client
        .put_item(PutItemInput {
            table_name: table_name,
            item: serde_dynamodb::to_hashmap(&put_item)?,
            ..Default::default()
        })
        .await
        .unwrap();
    println!("Write complete!");

    Ok(())
}
