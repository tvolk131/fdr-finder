use crate::podcast::{Podcast, PodcastTag};

mod meilisearch;

pub use meilisearch::SearchResult;

pub struct SearchBackend {
    meilisearch_backend_or: Option<meilisearch::MeilisearchBackend>, // Only `None` if running in mock mode.
}

impl SearchBackend {
    pub async fn new_prod(meilisearch_host: String, meilisearch_api_key: String) -> Self {
        Self {
            meilisearch_backend_or: Some(
                meilisearch::MeilisearchBackend::new(meilisearch_host, meilisearch_api_key).await,
            ),
        }
    }

    pub fn new_mock() -> Self {
        Self {
            meilisearch_backend_or: None,
        }
    }

    pub async fn search(
        &self,
        query_or: &Option<String>,
        tags: &[PodcastTag],
        limit_or: Option<usize>,
        offset: usize,
    ) -> SearchResult {
        match &self.meilisearch_backend_or {
            Some(meilisearch_backend) => {
                meilisearch_backend
                    .search(query_or, tags, limit_or.unwrap_or(99999999), offset)
                    .await
            }
            None => meilisearch::generate_mock_search_results(),
        }
    }

    pub async fn ingest_podcasts_or_panic(&self, podcasts: &[Podcast]) {
        match &self.meilisearch_backend_or {
            Some(meilisearch_backend) => {
                meilisearch_backend.ingest_podcasts_or_panic(podcasts).await;
            }
            None => {}
        };
    }
}
