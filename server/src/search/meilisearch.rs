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

    // TODO - Remove use of `block_on` here and use async instead.
    fn get_podcast_index(&self) -> Index {
        futures::executor::block_on(
            Client::new(self.host.clone(), self.api_key.clone()).get_or_create("podcasts"),
        )
        .unwrap()
    }

    // TODO - Remove use of `block_on` here and use async instead.
    pub fn search(&self, query: &str, limit: usize, offset: usize) -> Vec<Podcast> {
        let podcast_index = self.get_podcast_index();
        let results = futures::executor::block_on(
            podcast_index
                .search()
                .with_query(query)
                .with_offset(offset)
                .with_limit(limit)
                .execute::<Podcast>(),
        )
        .unwrap();
        results
            .hits
            .into_iter()
            .map(|result| result.result)
            .collect()
    }

    pub async fn ingest_podcasts_or_panic(&self, podcasts: &[Podcast]) {
        let podcast_index = self.get_podcast_index();
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
