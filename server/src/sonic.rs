use serde_json::Number;
use sonic_channel::*;
use std::sync::Arc;

use crate::{
    fdr_cache::FdrCache,
    mock::create_mock_podcast,
    podcast::{Podcast, PodcastNumber},
};

const FDR_COLLECTION: &str = "fdr";
const FDR_TITLE_BUCKET: &str = "title";

pub trait SearchBackend {
    fn search_by_title(&self, query: &str) -> Vec<&Arc<Podcast>>;
    fn suggest_by_title(&self, query: &str) -> Vec<String>;
}

pub struct SonicInstance {
    address: String,
    password: String,
    podcast_cache: Arc<FdrCache>,
}

// TODO - Handle all of the `unwrap` instances in this struct.
impl SearchBackend for SonicInstance {
    fn search_by_title(&self, query: &str) -> Vec<&Arc<Podcast>> {
        SearchChannel::start(&self.address, &self.password)
            .unwrap()
            .query(FDR_COLLECTION, FDR_TITLE_BUCKET, query)
            .unwrap()
            .into_iter()
            .filter_map(|podcast_num| {
                self.podcast_cache.get_podcast(&PodcastNumber::new(
                    Number::from_f64(podcast_num.parse().unwrap()).unwrap(),
                ))
            })
            .collect()
    }

    fn suggest_by_title(&self, query: &str) -> Vec<String> {
        let mut query_words: Vec<&str> = query.split(' ').collect();
        let last_word: &str = query_words.pop().unwrap_or(query);
        let prefix: String = query_words.join(" ");
        SearchChannel::start(&self.address, &self.password)
            .unwrap()
            .suggest_with_limit(FDR_COLLECTION, FDR_TITLE_BUCKET, last_word, 5)
            .unwrap()
            .into_iter()
            .map(|suggestion| format!("{} {}", prefix, suggestion).trim().to_string())
            .collect()
    }
}

// TODO - Handle all of the `unwrap` instances in this struct.
impl SonicInstance {
    pub fn new(address: String, password: String, podcast_cache: Arc<FdrCache>) -> Self {
        Self {
            address,
            password,
            podcast_cache,
        }
    }

    fn ingest(&self, podcast: &Podcast, ingest_channel: &IngestChannel) {
        match ingest_channel.push(
            FDR_COLLECTION,
            FDR_TITLE_BUCKET,
            &podcast.get_podcast_number().to_string(),
            podcast.get_title(),
        ) {
            Ok(_) => {}
            Err(err) => {
                println!(
                    "Failed to index podcast {}! Error: {}",
                    podcast.get_podcast_number().to_string(),
                    err
                );
            }
        };
    }

    pub fn ingest_all(&self) {
        let ingest_channel = IngestChannel::start(&self.address, &self.password).unwrap();
        self.podcast_cache
            .get_all_podcasts()
            .iter()
            .for_each(|podcast| self.ingest(podcast, &ingest_channel));
    }
}

pub struct MockSearchBackend {
    mock_podcasts: Vec<Arc<Podcast>>,
}

impl SearchBackend for MockSearchBackend {
    fn search_by_title(&self, _query: &str) -> Vec<&Arc<Podcast>> {
        self.mock_podcasts.iter().collect()
    }

    fn suggest_by_title(&self, _query: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        for i in 1..5 {
            suggestions.push(format!("Suggestion #{}", i));
        }
        suggestions
    }
}

impl Default for MockSearchBackend {
    fn default() -> Self {
        let mut mock_podcasts = Vec::new();
        for i in 1..20 {
            mock_podcasts.push(Arc::from(create_mock_podcast(i)));
        }

        Self { mock_podcasts }
    }
}
