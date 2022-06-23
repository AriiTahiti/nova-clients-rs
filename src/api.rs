use crate::market::*;
use crate::account::*;


pub struct Ftx {
    pub markets: MarketsClient,
    pub account: AccountClient,

}

impl Ftx {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Ftx {
            markets: MarketsClient::new(),
            account: AccountClient::new(api_key, secret_key),
        }
    }
}
