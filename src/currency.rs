use reqwest::blocking::Client;
use serde_json::Value;
use std::{
    error::Error,
    io::{self, Write},
};


pub const CURRENCY_LIST: [&str; 31] = [
    "AUD", "BGN", "BRL", "CAD", "CHF", "CNY", "CZK", "DKK", "EUR", "GBP", "HKD", "HUF", "IDR",
    "ILS", "INR", "ISK", "JPY", "KRW", "MXN", "MYR", "NOK", "NZD", "PHP", "PLN", "RON", "SEK",
    "SGD", "THB", "TRY", "USD", "ZAR",
];

pub fn currency_conversion(
    from_currency: String,
    to_currency: String,
    amount: u64,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://api.frankfurter.app/latest?amount={}&from={}&to={}",
        amount, from_currency, to_currency
    );

    let client = Client::new();
    let resp = client.get(&url).send().expect("unable to get response");
    let value: Value = resp.json().expect("unable to parse json");
    let currencies: Vec<&str> = to_currency.split(",").collect();
    let mut results = Vec::new();
    for currency_code in &currencies {
        if let Some(rate) = value["rates"].get(currency_code) {
            results.push(format!("{} {}", rate, currency_code));
        }
    }
    println!("\n{} {} = {}", amount, from_currency, results.join(", "));

    Ok(())
}

pub fn user_prompt(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        io::stdout().flush().expect("unable to flush");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read input");
        if !input.trim().is_empty() {
            return input.trim().to_string();
        }
    }
}

pub fn clr() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}

pub fn supported_currencies() {
    println!("Supported Currencies: ");
    for currency in CURRENCY_LIST {
        println!("{}", currency);
    }
}
