use crate::podcast::Podcast;
use meilisearch_sdk::{client::Client, indexes::Index, progress::UpdateStatus};

pub struct MeilisearchBackend {
    podcast_index: Index
}

// TODO - Implement the methods below.
impl MeilisearchBackend {
    pub async fn new(host: String, api_key: String) -> Self {
        Self { podcast_index: Client::new(host, api_key).get_or_create("podcasts").await.unwrap() }
    }

    pub async fn search(&self, query: &str, limit: usize, offset: usize) -> Vec<Podcast> {
        let results = self.podcast_index
            .search()
            .with_query(query)
            .with_offset(offset)
            .with_limit(limit)
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
        let progress = self.podcast_index.add_documents(podcasts, Some("podcast_number_hash")).await
        .unwrap();
        let status = progress.wait_for_pending_update(None, None).await.unwrap().unwrap();
        if let UpdateStatus::Failed { .. } = status {
            panic!("Meilisearch ingestion failed: {:?}", status)
        };
    }
}
