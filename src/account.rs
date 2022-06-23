use crate::client::Client;
use crate::errors::*;
use crate::model;

#[derive(Clone)]
pub struct AccountClient {
    client: Client,
}

impl AccountClient {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        AccountClient {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_account(&self) -> Result<model::AccountInfo> {
        let payload: String = format!("{}", "{}");
        let data = self.client.get_signed("/account".into(), payload, None)?;
        let account: model::Account = serde_json::from_str(data.as_str())?;
        Ok(account.result)
    }

}