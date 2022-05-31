mod stock_service;

use async_std::prelude::*;
use chrono::prelude::*;
use clap::Parser;
use futures::stream::FuturesUnordered;



use crate::stock_service::yahoo_stock_service as yahoo_stock_service;


#[derive(Parser, Debug)]
#[clap(
    version = "1.2",
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
    let mut futures = FuturesUnordered::new();
    for symbol in opts.symbols.split(',') {
        let future = yahoo_stock_service.fetch_stock_quotes_for_symbol(symbol, &from, &to);
        futures.push(future);
    }
    while let Some(response_option) = futures.next().await {
        if let Some(service_response) = response_option {
            println!("{}", service_response);
        }        
    }
    Ok(())
}
