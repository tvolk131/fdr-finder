use crate::podcast::Podcast;
use meilisearch_sdk::{client::Client, indexes::Index, progress::UpdateStatus};

pub struct MeilisearchBackend {
    host: String,
    api_key: String,
}

// TODO - Implement the methods below.
impl MeilisearchBackend {
    pub async fn new(host: String, api_key: String) -> Self {
        Self { host, api_key }
    }

    async fn get_podcast_index(&self) -> Index {
        Client::new(self.host.clone(), self.api_key.clone())
            .get_or_create("podcasts")
            .await
            .unwrap()
    }

    pub async fn search(&self, query: &str) -> Vec<Podcast> {
        let podcast_index = self.get_podcast_index().await;
        let results = podcast_index
            .search()
            .with_query(query)
            .with_limit(99999)
            .execute::<Podcast>()
            .await
            .unwrap();
        results
            .hits
            .into_iter()
            .map(|result| result.result)
            .collect()
    }

    pub async fn ingest_podcasts_or_panic(&self, podcasts: &[Podcast]) {
        let podcast_index = self.get_podcast_index().await;
        let progress = podcast_index
            .add_documents(podcasts, Some("podcast_number_hash"))
            .await
            .unwrap();
        let status = progress
            .wait_for_pending_update(None, None)
            .await
            .unwrap()
            .unwrap();
        if let UpdateStatus::Failed { .. } = status {
            panic!("Meilisearch ingestion failed: {:?}", status)
        };
    }
}
