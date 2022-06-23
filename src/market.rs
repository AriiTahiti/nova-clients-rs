use crate::client::Client;
use crate::model;
use crate::errors::*;


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
        let candles: model::ResultData<Vec<model::MarketData>> =
            serde_json::from_str(data.as_str())?;
        Ok(candles.result)
    }
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn get_historical() {

        let market = MarketsClient::new();

        let candle = market.get_historical_prices("BTC-PERP", 300, 0, 1655986295);

        match candle {
            Ok(c) => {
                println!("candles: {:?}", c);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }


    

}

