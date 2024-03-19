//use tokio::sync::broadcast::error;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    dotenv().ok();

    // Retrieve API key and base URL from environment variables
    let api_key: String = env::var("CURRENCY_API_KEY").expect("CURRENCY_API_KEY not set");
    let api_url_base: String = env::var("CURRENCY_API_URL_BASE").expect("CURRENCY_API_URL_BASE not set");

    // Define base and target currencies
    let base_currency: &str = "PLN";
    let target_currency: &str = "USD";

    // Construct the URL for fetching currency exchange rates
    let url: String = format!("{}?apikey={}&currencies={}%2C{}", api_url_base, api_key, base_currency, target_currency);
    

    let currencies = reqwest::Client::new()
    .get(&url)
    .send()
    .await?
    .text()
    .await?;

    println!("{:?}", currencies);

    Ok(())

}
