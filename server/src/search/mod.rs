use crate::{podcast::{Podcast, PodcastTag}, mock::create_mock_podcast};

mod meilisearch;

fn generate_mock_search_results() -> Vec<Podcast> {
    let mut mock_podcasts = Vec::new();
    for i in 1..20 {
        mock_podcasts.push(create_mock_podcast(i));
    }
    mock_podcasts
}

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
    ) -> Vec<Podcast> {
        match &self.meilisearch_backend_or {
            Some(meilisearch_backend) => {
                meilisearch_backend
                    .search(query_or, tags, limit_or.unwrap_or(99999999), offset)
                    .await
            }
            None => generate_mock_search_results(),
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
