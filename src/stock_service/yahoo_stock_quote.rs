use std::io::{ErrorKind, Error};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use yahoo_finance_api as yahoo;

use super::stock_service::{TickerData, StockQuote};

struct YahooStockQuote {
    connection: yahoo::YahooConnector,
    stock_quote: Option<TickerData>,
}

impl YahooStockQuote {
    fn new() -> Self {
        Self { 
            connection: yahoo::YahooConnector::new(), 
            stock_quote: None, 
        }
    }

    ///
    /// Retrieve data from a data source and extract the closing prices. Errors during download are mapped onto io::Errors as InvalidData.
    ///
    async fn fetch_closing_data(
        &self,
        symbol: &str,
        beginning: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> std::io::Result<Vec<f64>> {
        let connection = yahoo::YahooConnector::new();
        let response = connection
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
}

#[async_trait]
impl StockQuote for YahooStockQuote {
    async fn fetch_stock_quotes(&mut self) {
        todo!()
    }


    async fn update_stock_quotes(&mut self)  {
        todo!()
    }

}