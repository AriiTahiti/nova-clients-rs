use crate::client::Client;
use crate::errors::*;
use crate::model;

#[derive(Clone)]
pub struct MarketsClient {
    client: Client,
}

impl MarketsClient {
    
    pub fn new() -> Self {
        MarketsClient {
            client: Client::new(None, None),
        }
    }

    pub fn get_historical_prices<S, R, ST, ET>(
        &self,
        symbol: S,
        resolution: R,
        start_time: ST,
        end_time: ET,
    ) -> Result<Vec<model::MarketData>>
    where
        S: Into<String>,
        R: Into<i64>,
        ST: Into<i64>,
        ET: Into<i64>,
    {
        let endpoint: String = format!(
            "/markets/{}/candles?resolution={}&start_time={}&end_time={}",
            symbol.into(),
            resolution.into(),
            start_time.into(),
            end_time.into()
        );
        let data = self.client.get(endpoint, "".into())?;
        let candles: model::ResultData<Vec<model::CandleInfo>> =
            serde_json::from_str(data.as_str())?;
        Ok(candles.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_market() {
        let client = MarketsClient::new();
        let ticker = client.get_market("BTC-PERP");
        println!("{:#?}", ticker);
    }
}