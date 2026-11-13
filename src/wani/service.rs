use crate::wani::state::Wani;

use super::api::WaniClient;

pub struct WaniService {
    pub client: WaniClient,
    pub state: Wani,
}

impl WaniService {
    pub fn new() -> Self {
        let client = WaniClient::new();
        let state = Wani::default();

        Self { client, state }
    }
    async fn update_reviews(&mut self) {
        let summary = self.client.get_summary().await.unwrap();
        for &id in &summary.data.reviews.first().unwrap().subject_ids {
            self.state
                .subjects
                .push(self.client.get_subject(id).await.unwrap());
        }
    }
}
