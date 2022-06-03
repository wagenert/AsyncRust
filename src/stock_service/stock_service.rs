use std::fmt::Display;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait StockService {
    async fn fetch_stock_quotes_for_symbol(
        &self,
        symbol: &str,
        from: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Option<TickerData>;
}

#[async_trait]
pub trait StockQuote {
    async fn fetch_stock_quotes(
        &mut self
    );

    async fn update_stock_quotes(
        &mut self
    );
}

pub struct TickerData {
    pub symbol: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub max: f64,
    pub min: f64,
    pub diff: f64,
    pub last: f64,
    pub sma: Vec<f64>,
    last_update: Option<u64>,
}

impl TickerData {
    pub fn new(
        symbol: &str,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
        max_price: f64,
        min_price: f64,
        price_diff: f64,
        last_price: f64,
        sma: Vec<f64>,
        last_update: u64
    ) -> Self {
        TickerData {
            symbol: symbol.to_string(),
            from: from_date,
            to: to_date,
            max: max_price,
            min: min_price,
            diff: price_diff,
            last: last_price,
            sma: sma,
            last_update: Some(last_update),
        }
    }
}

impl Display for TickerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
            self.from.to_rfc3339(),
            self.symbol,
            self.last,
            self.diff * 100.0,
            self.min,
            self.max,
            self.sma.last().unwrap_or(&0.0)
        )
    }
}