use dotenv::dotenv;
use lambda::handler_fn;

// lib
use rust_weather_cron::{handler, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    // Put any one-time initialisation code up here
    // Before lambda::run is called!
    println!("Lambda starting");
    let func = handler_fn(handler);
    let _ = lambda::run(func).await.unwrap();
    println!("Lambda finished");

    Ok(())
}
