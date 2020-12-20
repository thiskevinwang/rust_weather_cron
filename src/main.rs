use chrono::prelude::*;
use dotenv::dotenv;
use lambda::{handler_fn, Context};
use reqwest;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};
use serde_json::Value;
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    // Put any one-time initialisation code up here
    // Before lambda::run is called!
    let func = handler_fn(func);
    lambda::run(func).await?;

    Ok(())
}

async fn func(_: Value, _: Context) -> Result<(), Error> {
    let location_id = "2171775";
    let api_key = env::var("API_KEY")?;

    let url = format!("http://dataservice.accuweather.com/forecasts/v1/hourly/12hour/{location_id}?apikey={api_key}", location_id=location_id, api_key=api_key);
    let resp = reqwest::get(&url).await?;
    let json = resp.json::<Vec<Forecast>>().await?;

    let client = DynamoDbClient::new(Region::UsEast1);

    // Format for Dynamo 2018-01-26T18:30:09.453Z
    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let put_item: Item = Item {
        pk: "icepond".to_string(),
        sk: now,
        forecasts: json,
    };

    client
        .put_item(PutItemInput {
            table_name: "weather".to_string(),
            item: serde_dynamodb::to_hashmap(&put_item)?,
            ..Default::default()
        })
        .await?;

    Ok(())
}
