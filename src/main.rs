mod exchange_rate;
mod converter;

use std::env;
use dotenv::dotenv;

use exchange_rate::ApiExchangeRateProvider;
use converter::CurrencyConverter;


fn exchange_currencies(rate: f64, amount: f64) -> f64 {
    amount * rate
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Usage: exchange <amount> <from_currency> to <to_currency>");
        return Ok(());
    }

    let amount: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount");
            return Ok(());
        }
    };

    let base_currency = &args[2].to_uppercase();
    let target_currency = &args[4].to_uppercase();

    let api_url = env::var("CURRENCY_API_URL_BASE").expect("CURRENCY_API_URL_BASE not set");
    let exchange_rate_provider = ApiExchangeRateProvider::new(api_url);

    let converter = CurrencyConverter::new(exchange_rate_provider);

    // Example: Convert 100 USD to EUR
    match converter.convert(amount, &base_currency, &target_currency).await {
        Ok(result) => println!("Converted amount: {:.2} {}", result, target_currency),
        Err(err) => eprintln!("Error: {}", err),
    }


    Ok(())
}
