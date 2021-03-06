use crate::mock::create_mock_podcast;
use crate::podcast::{Podcast, PodcastTag};
use meilisearch_sdk::{client::Client, indexes::Index, progress::UpdateStatus};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct MeilisearchBackend {
    client: Arc<Client>,
    podcast_index: Arc<Index>,
}

impl MeilisearchBackend {
    pub async fn new(
        host: String,
        api_key: String,
    ) -> Result<Self, meilisearch_sdk::errors::Error> {
        let client = Client::new(host, api_key);
        let podcast_index = client.get_or_create("podcasts").await?;
        Ok(Self {
            client: Arc::from(client),
            podcast_index: Arc::from(podcast_index),
        })
    }

    /// Clears and rebuilds the underlying Meilisearch index.
    pub async fn reset_index(&self) -> Option<meilisearch_sdk::errors::Error> {
        match self.client.delete_index_if_exists("podcasts").await {
            Ok(_) => {}
            Err(err) => return Some(err),
        };
        match self.client.get_or_create("podcasts").await {
            Ok(_index) => {} // No need to reassign index since, under the hood, the struct just contains a few string properties such as the host and index name.
            Err(err) => return Some(err),
        };
        match self
            .podcast_index
            .set_filterable_attributes(["tags", "lengthInSeconds"])
            .await
        {
            Ok(_) => {} // TODO - The value here is of type meilisearch_sdk::progress::Progress. This type does not guarantee that the operation completed, but rather is a way to check its progress. Therefore, we should actually be using the value here to further ensure that the operation successfully completes.
            Err(err) => return Some(err),
        };
        match self
            .podcast_index
            .set_sortable_attributes(["podcastNumber"])
            .await
        {
            Ok(_) => {} // TODO - The value here is of type meilisearch_sdk::progress::Progress. This type does not guarantee that the operation completed, but rather is a way to check its progress. Therefore, we should actually be using the value here to further ensure that the operation successfully completes.
            Err(err) => return Some(err),
        };
        None
    }

    pub async fn search(
        &self,
        query_or: &Option<String>,
        tags: &[PodcastTag],
        limit: usize,
        offset: usize,
        min_length_seconds: Option<usize>,
        max_length_seconds: Option<usize>,
    ) -> SearchResult {
        let mut search_request = self.podcast_index.search();

        let filter = Self::create_meilisearch_filter(tags, min_length_seconds, max_length_seconds);
        if !filter.is_empty() {
            search_request.with_filter(&filter);
        }

        search_request.with_sort(&["podcastNumber:desc"]);

        match query_or {
            Some(query) => {
                search_request.with_query(query);
            }
            None => {}
        };

        search_request.with_offset(offset).with_limit(limit);

        let results = search_request.execute::<Podcast>().await.unwrap();

        SearchResult {
            hits: results
                .hits
                .into_iter()
                .map(|result| result.result)
                .collect(),
            total_hits: results.nb_hits,
            total_hits_is_approximate: !results.exhaustive_nb_hits,
            processing_time_ms: results.processing_time_ms,
        }
    }

    fn create_meilisearch_filter(
        tags: &[PodcastTag],
        min_length_seconds: Option<usize>,
        max_length_seconds: Option<usize>,
    ) -> String {
        let mut filter_elements: Vec<String> = Vec::new();

        if !tags.is_empty() {
            filter_elements.push(
                tags.iter()
                    .map(|tag| format!("tags = \"{}\"", tag.clone_to_string()))
                    .collect::<Vec<String>>()
                    .join(" AND "),
            );
        }

        if let Some(min_length_seconds) = min_length_seconds {
            filter_elements.push(format!("lengthInSeconds > {}", min_length_seconds - 1));
        }

        if let Some(max_length_seconds) = max_length_seconds {
            filter_elements.push(format!("lengthInSeconds < {}", max_length_seconds + 1));
        }

        filter_elements.join(" AND ")
    }

    pub async fn ingest_podcasts_or_panic(&self, podcasts: impl Iterator<Item = &Podcast>) {
        let mut cloned_podcasts: Vec<Podcast> = podcasts.cloned().collect();
        // Since the first items that are indexed have highest priority, reversing
        // the order ensures that the latest podcasts are returned first.
        cloned_podcasts.reverse();
        let progress = self
            .podcast_index
            .add_documents(&cloned_podcasts, Some("podcastNumberHash"))
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

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub hits: Vec<Podcast>,
    total_hits: usize,
    total_hits_is_approximate: bool,
    processing_time_ms: usize,
}

// TODO - Abstract this into a procedural macro along with all other Responder impl blocks in other structs.
impl<'r> rocket::response::Responder<'r, 'static> for SearchResult {
    fn respond_to(
        self,
        _request: &'r rocket::request::Request,
    ) -> Result<rocket::response::Response<'static>, rocket::http::Status> {
        let json_string = serde_json::json!(self).to_string();
        rocket::Response::build()
            .header(rocket::http::ContentType::JSON)
            .sized_body(json_string.len(), std::io::Cursor::new(json_string))
            .ok()
    }
}

impl SearchResult {
    pub fn get_hits(&self) -> &[Podcast] {
        &self.hits
    }

    pub fn take_hits(self) -> Vec<Podcast> {
        self.hits
    }
}

pub fn generate_mock_search_results() -> SearchResult {
    let mut mock_podcasts = Vec::new();
    for i in 1..20 {
        mock_podcasts.push(create_mock_podcast(i));
    }
    let total_hits = mock_podcasts.len();
    SearchResult {
        hits: mock_podcasts,
        total_hits,
        total_hits_is_approximate: false,
        processing_time_ms: 1234,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_meilisearch_filter() {
        assert_eq!(
            MeilisearchBackend::create_meilisearch_filter(&Vec::new(), None, None),
            ""
        );
        assert_eq!(
            MeilisearchBackend::create_meilisearch_filter(&Vec::new(), Some(1), None),
            "lengthInSeconds > 0"
        );
        assert_eq!(
            MeilisearchBackend::create_meilisearch_filter(&Vec::new(), Some(1), Some(2)),
            "lengthInSeconds > 0 AND lengthInSeconds < 3"
        );
        assert_eq!(
            MeilisearchBackend::create_meilisearch_filter(
                &[PodcastTag::new("hello world".to_string())],
                None,
                None
            ),
            "tags = \"hello world\""
        );
        assert_eq!(
            MeilisearchBackend::create_meilisearch_filter(
                &[
                    PodcastTag::new("foo".to_string()),
                    PodcastTag::new("bar".to_string())
                ],
                None,
                None
            ),
            "tags = \"foo\" AND tags = \"bar\""
        );
        assert_eq!(
            MeilisearchBackend::create_meilisearch_filter(
                &[PodcastTag::new("hello world".to_string())],
                Some(1),
                Some(2)
            ),
            "tags = \"hello world\" AND lengthInSeconds > 0 AND lengthInSeconds < 3"
        );
        assert_eq!(
            MeilisearchBackend::create_meilisearch_filter(
                &[
                    PodcastTag::new("foo".to_string()),
                    PodcastTag::new("bar".to_string())
                ],
                Some(1),
                Some(2)
            ),
            "tags = \"foo\" AND tags = \"bar\" AND lengthInSeconds > 0 AND lengthInSeconds < 3"
        );
    }
}
