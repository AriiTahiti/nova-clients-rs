use crate::market::*;



pub struct Ftx {
    pub markets: MarketsClient,
}

impl Ftx {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Ftx {
            markets: MarketsClient::new(),
        }
    }
}
