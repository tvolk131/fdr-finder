use meilisearch_sdk::{client::Client, indexes::Index};
use crate::podcast::Podcast;

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

    async fn get_podcast_index(&self) -> Index {
        Client::new(self.host.clone(), self.api_key.clone()).get_or_create("podcasts").await.unwrap()
    }

    pub async fn search(&self, query: &str) -> Vec<Podcast> {
        let podcast_index = self.get_podcast_index().await;
        let results = podcast_index.search().with_query(query).with_limit(99999).execute::<Podcast>().await.unwrap();
        results.hits.into_iter().map(|result| result.result).collect()
    }

    pub async fn ingest_podcasts(&self, podcasts: &[Podcast]) {
        let podcast_index = self.get_podcast_index().await;
        podcast_index.add_documents(podcasts, Some("podcast_number")).await.unwrap();
    }
}