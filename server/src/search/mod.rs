use crate::podcast::{Podcast, PodcastTag};

mod cache;
mod meilisearch;

pub use meilisearch::SearchResult;

pub struct SearchBackend {
    meilisearch_backend_or: Option<meilisearch::MeilisearchBackend>, // Only `None` if running in mock mode.
    search_cache: cache::SearchCache,
}

impl SearchBackend {
    pub async fn new_prod(meilisearch_host: String, meilisearch_api_key: String) -> Self {
        Self {
            meilisearch_backend_or: Some(
                meilisearch::MeilisearchBackend::new(meilisearch_host, meilisearch_api_key).await,
            ),
            search_cache: cache::SearchCache::new(10000),
        }
    }

    pub fn new_mock() -> Self {
        Self {
            meilisearch_backend_or: None,
            search_cache: cache::SearchCache::new(0),
        }
    }

    pub async fn search(
        &self,
        query_or: &Option<String>,
        tags: &[PodcastTag],
        limit_or: Option<usize>,
        offset: usize,
        minLengthSeconds: Option<usize>,
        maxLengthSeconds: Option<usize>,
    ) -> SearchResult {
        match &self.meilisearch_backend_or {
            Some(meilisearch_backend) => {
                let mut response = self.search_cache
                    .search(query_or, tags, limit_or, offset, meilisearch_backend)
                    .await;
                response.hits = response.hits.into_iter().filter(|hit| {
                    match minLengthSeconds {
                        Some(minLengthSeconds) => {
                            if hit.length_in_seconds < minLengthSeconds {
                                return false
                            }
                        },
                        None => {}
                    };
                    match maxLengthSeconds {
                        Some(maxLengthSeconds) => {
                            if hit.length_in_seconds > maxLengthSeconds {
                                return false
                            }
                        },
                        None => {}
                    };
                    true
                }).collect();
                response
            }
            None => meilisearch::generate_mock_search_results(),
        }
    }

    pub async fn ingest_podcasts_or_panic(&self, podcasts: impl Iterator<Item = &Podcast>) {
        match &self.meilisearch_backend_or {
            Some(meilisearch_backend) => {
                meilisearch_backend.ingest_podcasts_or_panic(podcasts).await;
            }
            None => {}
        };
    }
}
