use std::{collections::HashMap, env};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CurrencyData {
    meta: Option<Meta>,
    data: HashMap<String, Currency>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    last_updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Currency {
    code: String,
    value: f64,
}

async fn get_exchange_rate(base_currency: &str, target_currency: &str) -> Result<f64, reqwest::Error>{
    dotenv().ok();

    let api_key = env::var("CURRENCY_API_KEY").expect("CURRENCY_API_KEY not set");
    let api_url_base = env::var("CURRENCY_API_URL_BASE").expect("CURRENCY_API_URL_BASE not set");

    let url = format!("{}/latest?access_key={}&symbols={},{}", api_url_base, api_key, base_currency, target_currency);

    let response = reqwest::get(&url).await?;
    let currency_data: CurrencyData = response.json().await?;

     let currency = currency_data.data.get(target_currency);

        Ok(currency.unwrap().value)

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let source_currency = "PLN";
    let target_currency = "USD";
    let amount: f64 = 3.6;
    
    let exchange_rate = match get_exchange_rate(source_currency, target_currency).await {
        Ok(rate) => rate,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    let converted_amount = amount * exchange_rate;
    println!("Converted amount: {} {} (Exchange rate: {})", converted_amount, target_currency, exchange_rate);

    Ok(())
}
