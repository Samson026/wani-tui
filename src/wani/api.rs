use std::env;

use reqwest::{Client, Error, Result, header::AUTHORIZATION};

use crate::wani::types::{Subject, Summary};

pub struct WaniClient {
    client: reqwest::Client,
    token: String,
}

impl WaniClient {
    pub fn new() -> Self {
        let token = env::var("WANI").unwrap();
        let client = Client::new();

        Self { client, token }
    }

    pub async fn get_summary(&self) -> Result<Summary> {
        self.client
            .get("https://api.wanikani.com/v2/summary")
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_subject(&self, id: u32) -> Result<Subject> {
        self.client
            .get(format!("https://api.wanikani.com/v2/subjects/{id}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .send()
            .await?
            .json()
            .await
    }
}
