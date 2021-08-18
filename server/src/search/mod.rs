use crate::podcast::Podcast;
use crate::fdr_cache::FdrCache;
use std::sync::Arc;

mod meilisearch;
mod mock;
mod sonic;

pub struct SearchBackend {
    meilisearch_backend_or: Option<meilisearch::MeilisearchBackend>, // Only `None` if running in mock mode.
    mock_backend_or: Option<mock::MockBackend>, // Only `Some` if running in mock mode.
    sonic_backend_or: Option<sonic::SonicBackend> // Only `None` if running in mock mode.
}

impl SearchBackend {
    pub async fn new_prod(sonic_uri: String, sonic_password: String, meilisearch_host: String, meilisearch_api_key: String, fdr_cache: Arc<FdrCache>) -> Self {
        Self {
            meilisearch_backend_or: Some(meilisearch::MeilisearchBackend::new(meilisearch_host, meilisearch_api_key).await),
            mock_backend_or: None,
            sonic_backend_or: Some(sonic::SonicBackend::new(sonic_uri, sonic_password, fdr_cache))
        }
    }

    pub fn new_mock() -> Self {
        Self {
            meilisearch_backend_or: None,
            mock_backend_or: Some(mock::MockBackend::default()),
            sonic_backend_or: None
        }
    }

    pub async fn search_by_title(&self, query: &str) -> Vec<&Arc<Podcast>> {
        // TODO - Implement.
        Vec::new()
    }

    pub async fn suggest_by_title(&self, query: &str) -> Vec<String> {
        // TODO - Implement.
        Vec::new()
    }

    pub async fn ingest_podcasts(&self, podcasts: &[Podcast]) {
        match &self.meilisearch_backend_or {
            Some(meilisearch_backend) => {
                meilisearch_backend.ingest_podcasts(podcasts);
            },
            None => {}
        };

        match &self.sonic_backend_or {
            Some(sonic_backend) => {
                sonic_backend.ingest_podcasts(podcasts);
            },
            None => {}
        };
    }
}
