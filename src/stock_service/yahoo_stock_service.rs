use std::io::{Error, ErrorKind};

use chrono::{DateTime, Utc};
use yahoo_finance_api as yahoo;

use crate::stock_service::async_stock_signals::{MaxPrice, MinPrice, PriceDifference, WindowedSMA};

use super::{async_stock_signals::AsyncStockSignal, stock_service::StockServiceResponse};

pub(crate) struct YahooStockService {
    max: MaxPrice,
    min: MinPrice,
    price_diff: PriceDifference,
    sma: WindowedSMA,
}

impl YahooStockService {
    pub fn new() -> Self {
        YahooStockService {
            max: MaxPrice {},
            min: MinPrice {},
            price_diff: PriceDifference {},
            sma: WindowedSMA { window_size: 30 },
        }
    }

    pub async fn fetch_stock_quotes_for_symbol(
        &self,
        symbol: &str,
        from: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Option<StockServiceResponse> {
        if let Ok(closes) = self.fetch_closing_data(&symbol, &from, &to).await {
            if !closes.is_empty() {
                // min/max of the period. unwrap() because those are Option types
                let period_max: f64 = self.max.calculate(&closes).await.unwrap();
                let period_min: f64 = self.min.calculate(&closes).await.unwrap();
                let last_price = *closes.last().unwrap_or(&0.0);
                let (_, pct_change) = self
                    .price_diff
                    .calculate(&closes)
                    .await
                    .unwrap_or((0.0, 0.0));
                let sma = self.sma.calculate(&closes).await.unwrap_or_default();
                return Some(StockServiceResponse::new(
                    symbol, *from, *to, period_max, period_min, pct_change, last_price, sma,
                ));
            } else {
                return None;
            }
        } else {
            return None;
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
