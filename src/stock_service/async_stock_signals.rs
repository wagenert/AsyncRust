use async_trait::async_trait;

use crate::stock_service::calc::{price_diff, min, max, n_window_sma};

///
/// A trait to provide a common interface for all signal calculations.
///
#[async_trait]
pub trait AsyncStockSignal {

    ///
    /// The signal's data type.
    ///
    type SignalType;

    ///
    /// Calculate the signal on the provided series.
    ///
    /// # Returns
    ///
    /// The signal (using the provided type) or `None` on error/invalid data.
    ///
    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType>;
}

pub(crate) struct PriceDifference;

#[async_trait]
impl AsyncStockSignal for PriceDifference {
    type SignalType = (f64, f64);

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        price_diff(series)
    }
}

pub(crate) struct MinPrice;

#[async_trait]
impl AsyncStockSignal for MinPrice {
    type SignalType = f64;

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        min(series)
    }
}

pub(crate) struct MaxPrice;

#[async_trait]
impl AsyncStockSignal for MaxPrice {
    type SignalType = f64;

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        max(series)
    }
}

pub(crate) struct WindowedSMA {
    window_size: usize,
}

#[async_trait]
impl AsyncStockSignal for WindowedSMA {
    type SignalType = Vec<f64>;

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        n_window_sma(self.window_size, series)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use async_std::task;

    use super::*;

    #[test]
    fn test_PriceDifference_calculate() {
        let signal = PriceDifference {};
        assert_eq!(task::block_on(signal.calculate(&[])), None);
        assert_eq!(task::block_on(signal.calculate(&[1.0])), Some((0.0, 0.0)));
        assert_eq!(task::block_on(signal.calculate(&[1.0, 0.0])), Some((-1.0, -1.0)));
        assert_eq!(
            task::block_on(signal.calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])),
            Some((8.0, 4.0))
        );
        assert_eq!(
            task::block_on(signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0])),
            Some((1.0, 1.0))
        );
    }

    #[test]
    fn test_MinPrice_calculate() {
        let signal = MinPrice {};
        assert_eq!(task::block_on(signal.calculate(&[])), None);
        assert_eq!(task::block_on(signal.calculate(&[1.0])), Some(1.0));
        assert_eq!(task::block_on(signal.calculate(&[1.0, 0.0])), Some(0.0));
        assert_eq!(
            task::block_on(signal.calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])),
            Some(1.0)
        );
        assert_eq!(
            task::block_on(signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0])),
            Some(0.0)
        );
    }

    #[test]
    fn test_MaxPrice_calculate() {
        let signal = MaxPrice {};
        assert_eq!(task::block_on(signal.calculate(&[])), None);
        assert_eq!(task::block_on(signal.calculate(&[1.0])), Some(1.0));
        assert_eq!(task::block_on(signal.calculate(&[1.0, 0.0])), Some(1.0));
        assert_eq!(
            task::block_on(signal.calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])),
            Some(10.0)
        );
        assert_eq!(
            task::block_on(signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0])),
            Some(6.0)
        );
    }

    #[test]
    fn test_WindowedSMA_calculate() {
        let series = vec![2.0, 4.5, 5.3, 6.5, 4.7];

        let signal = WindowedSMA { window_size: 3 };
        assert_eq!(
            task::block_on(signal.calculate(&series)),
            Some(vec![3.9333333333333336, 5.433333333333334, 5.5])
        );

        let signal = WindowedSMA { window_size: 5 };
        assert_eq!(task::block_on(signal.calculate(&series)), Some(vec![4.6]));

        let signal = WindowedSMA { window_size: 10 };
        assert_eq!(task::block_on(signal.calculate(&series)), Some(vec![]));
    }
}
