mod stock_service;

use async_std::prelude::*;
use chrono::prelude::*;
use clap::Parser;



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
            println!("{}", service_response);
        }
    }
    Ok(())
}
