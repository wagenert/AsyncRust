use std::fmt::Display;

use chrono::{DateTime, Utc};

pub struct StockServiceResponse {
    pub symbol: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub max: f64,
    pub min: f64,
    pub diff: f64,
    pub last: f64,
    pub sma: Vec<f64>,
}

impl StockServiceResponse {
    pub fn new(
        symbol: &str,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
        max_price: f64,
        min_price: f64,
        price_diff: f64,
        last_price: f64,
        sma: Vec<f64>
    ) -> Self {
        StockServiceResponse {
            symbol: symbol.to_string(),
            from: from_date,
            to: to_date,
            max: max_price,
            min: min_price,
            diff: price_diff,
            last: last_price,
            sma: sma,
        }
    }
}

impl Display for StockServiceResponse {
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