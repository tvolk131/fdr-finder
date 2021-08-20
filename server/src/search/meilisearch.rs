use crate::podcast::{Podcast, PodcastTag};
use meilisearch_sdk::{client::Client, indexes::Index, progress::UpdateStatus};

pub struct MeilisearchBackend {
    podcast_index: Index,
}

// TODO - Implement the methods below.
impl MeilisearchBackend {
    pub async fn new(host: String, api_key: String) -> Self {
        let podcast_index = Client::new(host, api_key)
            .get_or_create("podcasts")
            .await
            .unwrap();
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
    ) -> Vec<Podcast> {
        // These three lines are pretty weird and gross, but `with_facet_filters` specifically accepts &[&[&str]] so we need to create the facet strings and then borrow them.
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
            Some(query) => { search_request.with_query(query); },
            None => {}
        };

        search_request
            .with_offset(offset)
            .with_limit(limit);

        let results = search_request.execute::<Podcast>().await.unwrap();
        results
            .hits
            .into_iter()
            .map(|result| result.result)
            .collect()
    }

    pub async fn ingest_podcasts_or_panic(&self, podcasts: &[Podcast]) {
        let progress = self
            .podcast_index
            .add_documents(podcasts, Some("podcast_number_hash"))
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
