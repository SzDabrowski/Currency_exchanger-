use std::{collections::HashMap, env};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CurrencyData {
    meta: Meta,
    data: HashMap<String, Currency>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    #[serde(rename = "lastUpdatedAt")]
    last_updated_at: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct Currency {
    code: String,
    value: f64,
}

async fn get_exchange_rate(url: &str) -> Result<CurrencyData, reqwest::Error> {
    let response = reqwest::get(url).await?; // Use GET method to fetch data
    let currency_data = response.json().await?; // Deserialize JSON response

    Ok(currency_data)
}

fn serialize_url(base_currency: &str, target_currency: &str) -> String {
    let api_key = env::var("CURRENCY_API_KEY").expect("CURRENCY_API_KEY not set");
    let api_url_base = env::var("CURRENCY_API_URL_BASE").expect("CURRENCY_API_URL_BASE not set");

    format!("{}?apikey={}&currencies={}&base_currency={}", api_url_base, api_key, target_currency, base_currency)
}

fn exchange_currencies(rate: f64, amount: f64) -> f64 {
    amount * rate
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let base_currency = "PLN";
    let target_currency = "USD";

    let url = serialize_url(base_currency, target_currency);

   match get_exchange_rate(&url).await {
        Ok(data) => {
            // Extract currency value for the target currency
            if let Some(currency) = data.data.get(target_currency) {
                // println!("Value of {} in {}: {}", base_currency, target_currency, currency.value);
                println!("{}", exchange_currencies(currency.value, 200.0));
            } else {
                println!("Currency data not found for {}", target_currency);
            }
        }
        Err(err) => eprintln!("Error fetching currency data: {}", err),
    }

    Ok(())
}
