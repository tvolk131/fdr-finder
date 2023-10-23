use crate::mock::create_mock_podcast;
use crate::podcast::{Podcast, PodcastTag};
use meilisearch_sdk::tasks::Task;
use meilisearch_sdk::{client::Client, indexes::Index};
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

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
        let podcast_index = Self::get_wiped_podcast_index(&client).await?;
        Ok(Self {
            client: Arc::from(client),
            podcast_index: Arc::from(podcast_index),
        })
    }

    /// Clears and rebuilds the underlying Meilisearch index.
    pub async fn reset_index(&self) -> Result<(), meilisearch_sdk::errors::Error> {
        Self::get_wiped_podcast_index(&self.client).await?;
        Ok(())
    }

    /// Clears, rebuilds, and returns the underlying Meilisearch index.
    async fn get_wiped_podcast_index(
        client: &Client,
    ) -> Result<Index, meilisearch_sdk::errors::Error> {
        client
            .delete_index("podcasts")
            .await?
            .wait_for_completion(client, None, None)
            .await?;

        let podcast_index = client
            .create_index("podcasts", None)
            .await?
            .wait_for_completion(client, None, None)
            .await?
            .try_make_index(client)
            .unwrap();

        podcast_index
            .set_filterable_attributes(["tags", "lengthInSeconds"])
            .await?;
        podcast_index
            .set_sortable_attributes(["podcastNumber"])
            .await?;

        Ok(podcast_index)
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
            total_hits: results.estimated_total_hits,
            total_hits_is_approximate: true,
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
        let task = self
            .podcast_index
            .add_documents(&cloned_podcasts, Some("podcastNumberHash"))
            .await
            .unwrap()
            .wait_for_completion(&self.client, None, Some(Duration::from_secs(60)))
            .await
            .unwrap();
        assert!(matches!(task, Task::Succeeded { .. }));
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub hits: Vec<Podcast>,
    total_hits: usize,
    // TODO - Remove this field. It is always true.
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
