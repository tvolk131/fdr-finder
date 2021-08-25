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
        client.delete_index("podcasts").await.unwrap();
        let podcast_index = client.get_or_create("podcasts").await.unwrap();
        podcast_index
            .set_attributes_for_faceting(["tags"])
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
        // These three lines are pretty weird and gross, but `with_facet_filters` specifically
        // accepts &[&[&str]] so we need to create the facet strings and immediately borrow them.
        let tag_facet_strings: Vec<String> = tags
            .iter()
            .map(|tag| format!("tags:{}", tag.clone_to_string()))
            .collect();
        let grouped_tag_facet_strings: Vec<Vec<&str>> = tag_facet_strings
            .iter()
            .map(|tag_facet_string| vec![tag_facet_string.as_str()])
            .collect();
        let tag_facet_strings_search_arg: Vec<&[&str]> = grouped_tag_facet_strings
            .iter()
            .map(|tag_facet_group| tag_facet_group.as_slice())
            .collect();

        let mut search_request = self.podcast_index.search();

        if !tag_facet_strings_search_arg.is_empty() {
            search_request.with_facet_filters(tag_facet_strings_search_arg.as_slice());
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
        let mut cloned_podcasts: Vec<Podcast> = podcasts.map(|podcast| podcast.clone()).collect();
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    hits: Vec<Podcast>,
    total_hits: usize,
    total_hits_is_approximate: bool,
    processing_time_ms: usize,
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
