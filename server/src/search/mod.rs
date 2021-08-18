use crate::podcast::Podcast;
use crate::fdr_cache::FdrCache;
use std::sync::Arc;
use rocket::{http::RawStr, request::FromFormValue};

mod meilisearch;
mod mock;
mod sonic;

pub enum SearchBackendType {
    Meilisearch,
    Sonic
}

fn parse_search_backend_type_from_string(string: &str) -> Option<SearchBackendType> {
    if string == "meilisearch" {
        return Some(SearchBackendType::Meilisearch);
    }

    if string == "sonic" {
        return Some(SearchBackendType::Sonic);
    }

    None
}

impl<'v> FromFormValue<'v> for SearchBackendType {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<SearchBackendType, &'v RawStr> {
        match form_value.url_decode() {
            Ok(value) => match parse_search_backend_type_from_string(&value) {
                Some(search_backend_type) => Ok(search_backend_type),
                None => Err(form_value)
            },
            Err(_) => Err(form_value)
        }
    }
}

impl Default for SearchBackendType {
    fn default() -> Self {
        Self::Sonic
    }
}

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

    pub async fn search_by_title(&self, query: &str, search_backend_type: SearchBackendType) -> Vec<&Arc<Podcast>> {
        match search_backend_type {
            SearchBackendType::Meilisearch => {
                match &self.meilisearch_backend_or {
                    Some(meilisearch_backend) => meilisearch_backend.search_by_title(query).await,
                    None => match &self.mock_backend_or {
                        Some(mock_backend) => mock_backend.search_by_title(query),
                        None => Vec::new() // This line should never be hit since the constructors enforce that either the mock backend or the other backends are `Some`.
                    }
                }
            },
            SearchBackendType::Sonic => {
                match &self.sonic_backend_or {
                    Some(sonic_backend) => sonic_backend.search_by_title(query),
                    None => match &self.mock_backend_or {
                        Some(mock_backend) => mock_backend.search_by_title(query),
                        None => Vec::new() // This line should never be hit since the constructors enforce that either the mock backend or the other backends are `Some`.
                    }
                }
            }
        }
    }

    pub async fn suggest_by_title(&self, query: &str, search_backend_type: SearchBackendType) -> Vec<String> {
        match search_backend_type {
            SearchBackendType::Meilisearch => {
                Vec::new()
            },
            SearchBackendType::Sonic => {
                match &self.sonic_backend_or {
                    Some(sonic_backend) => sonic_backend.suggest_by_title(query),
                    None => match &self.mock_backend_or {
                        Some(mock_backend) => mock_backend.suggest_by_title(query),
                        None => Vec::new() // This line should never be hit since the constructors enforce that either the mock backend or the other backends are `Some`.
                    }
                }
            }
        }
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
