use crate::mock::create_mock_podcast;
use crate::podcast::{Podcast, PodcastTag};
use meilisearch_sdk::{client::Client, indexes::Index, progress::UpdateStatus};
use serde::Serialize;

pub struct MeilisearchBackend {
    podcast_index: Index,
}

impl MeilisearchBackend {
    pub async fn new(host: String, api_key: String) -> Self {
        let client = Client::new(host, api_key);
        client.delete_index_if_exists("podcasts").await.unwrap();
        let podcast_index = client.get_or_create("podcasts").await.unwrap();
        podcast_index
            .set_filterable_attributes(["tags"])
            .await
            .unwrap();
        Self { podcast_index }
    }

    pub async fn search(
        &self,
        query_or: &Option<String>,
        tags: &[PodcastTag],
        limit: usize,
        offset: usize,
    ) -> SearchResult {
        let mut search_request = self.podcast_index.search();

        let filter = format!("({})", tags.iter().map(|tag| format!("tags:{}", tag.clone_to_string())).collect::<Vec<String>>().join(" OR "));

        if !tags.is_empty() {
            search_request.with_filter(&filter);
        }

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
