use chrono::{DateTime, Utc};

pub struct StockServiceResponse {
    symbol: String,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    max: f64,
    min: f64,
    diff: f64,
    last: f64,
    sma: Vec<f64>,
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
            sma: Vec::new(),
        }
    }
}