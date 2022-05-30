mod stock_service;

use async_std::prelude::*;
use async_trait::async_trait;
use chrono::prelude::*;
use clap::Parser;
use std::io::{Error, ErrorKind};
use yahoo_finance_api as yahoo;

use crate::stock_service::yahoo_stock_service as yahoo_stock_service;


#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Claus Matzinger",
    about = "A Manning LiveProject: async Rust"
)]
struct Opts {
    #[clap(short, long, default_value = "AAPL,MSFT,UBER,GOOG")]
    symbols: String,
    #[clap(short, long)]
    from: String,
}

///
/// Retrieve data from a data source and extract the closing prices. Errors during download are mapped onto io::Errors as InvalidData.
///
async fn fetch_closing_data(
    symbol: &str,
    beginning: &DateTime<Utc>,
    end: &DateTime<Utc>,
) -> std::io::Result<Vec<f64>> {
    let provider = yahoo::YahooConnector::new();

    let response = provider
        .get_quote_history(symbol, *beginning, *end)
        .await
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut quotes = response
        .quotes()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    if !quotes.is_empty() {
        quotes.sort_by_cached_key(|k| k.timestamp);
        Ok(quotes.iter().map(|q| q.adjclose as f64).collect())
    } else {
        Ok(vec![])
    }
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let opts = Opts::parse();
    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");
    let to = Utc::now();

    let yahoo_stock_service = yahoo_stock_service::YahooStockService::new();

    // a simple way to output a CSV header
    println!("period start,symbol,price,change %,min,max,30d avg");
    for symbol in opts.symbols.split(',') {
        if let Some(service_response) = yahoo_stock_service.fetch_stock_quotes_for_symbol(symbol, &from, &to).await {
            println!(
                "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
                service_response.from.to_rfc3339(),
                service_response.symbol,
                service_response.last,
                service_response.diff * 100.0,
                service_response.min,
                service_response.max,
                service_response.sma.last().unwrap_or(&0.0)
            );
        }
    }
    Ok(())
}
