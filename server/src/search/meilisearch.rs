use meilisearch_sdk::{client::Client, indexes::Index};
use crate::podcast::Podcast;
use std::sync::Arc;

pub struct MeilisearchBackend {
    host: String,
    api_key: String
}

// TODO - Implement the methods below.
impl MeilisearchBackend {
    pub async fn new(host: String, api_key: String) -> Self {
        Self {
            host,
            api_key
        }
    }

    pub async fn search_by_title(&self, query: &str) -> Vec<&Arc<Podcast>> {
        let podcast_index: Index = Client::new(self.host.clone(), self.api_key.clone()).get_or_create("podcasts").await.unwrap();
        let results = podcast_index.search().with_query(query).with_limit(9999).execute::<Podcast>().await.unwrap();
        Vec::new()
    }

    pub async fn suggest_by_title(&self, query: &str) -> Vec<String> {
        Vec::new()
    }

    pub async fn ingest_podcasts(&self, podcasts: &[Podcast]) {
        let podcast_index: Index = Client::new(self.host.clone(), self.api_key.clone()).get_or_create("podcasts").await.unwrap();
        podcast_index.add_documents(podcasts, Some("podcast_number")).await;
    }
}