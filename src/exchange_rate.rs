use async_trait::async_trait;
use std::{collections::HashMap, env, fmt};
use reqwest::Error;
use serde::Deserialize;

#[async_trait]
pub trait ExchangeRateProvider {
    async fn get_exchange_rate(&self, base_currency: &str, target_currency: &str) -> Result<f64, ExchangeRateError>;
}

#[derive(Debug)]
pub enum ExchangeRateError {
    RequestError(Error),
    RateNotFound(String, String),
}

impl fmt::Display for ExchangeRateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExchangeRateError::RequestError(err) => write!(f, "Request error: {}", err),
            ExchangeRateError::RateNotFound(base_currency, target_currency) => {
                write!(f, "Exchange rate from {} to {} not found.", base_currency, target_currency)
            }
        }
    }
}

impl From<Error> for ExchangeRateError {
    fn from(error: Error) -> Self {
        ExchangeRateError::RequestError(error)
    }
}

pub struct ApiExchangeRateProvider {
    pub base_url: String,
    pub client: &'static reqwest::Client,
}

impl ApiExchangeRateProvider {
    pub fn new(api_url: String) -> Self {
        lazy_static::lazy_static! {
            static ref CLIENT: reqwest::Client = reqwest::Client::new();
        }

        ApiExchangeRateProvider {
            base_url: api_url.to_string(),
            client: &CLIENT,
        }
    }

    pub fn get_url(&self, base_currency: &str, target_currency: &str) -> String {
        let api_key = env::var("CURRENCY_API_KEY").expect("CURRENCY_API_KEY not set");
    
        format!("{}?apikey={}&currencies={}&base_currency={}", self.base_url, api_key, target_currency, base_currency)
    }
}

#[derive(Debug, Deserialize)]
struct CurrencyData {
    meta: Meta,
    data: HashMap<String, Currency>,
}

#[derive(Debug, Deserialize)]
struct Meta {
    last_updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Currency {
    code: String,
    value: f64,
}

#[async_trait]
impl ExchangeRateProvider for ApiExchangeRateProvider {
    async fn get_exchange_rate(&self, base_currency: &str, target_currency: &str) -> Result<f64, ExchangeRateError> {
        let response = self.client.get(self.get_url(base_currency, target_currency)).send().await?;
        let exchange_rates: CurrencyData = response.json().await?;
        match exchange_rates.data.get(target_currency) {
            Some(rate) => Ok(rate.value),
            None => Err(ExchangeRateError::RateNotFound(base_currency.to_string(), target_currency.to_string())),
        }
    }
}