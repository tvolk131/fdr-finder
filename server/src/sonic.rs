use serde_json::Number;
use sonic_channel::*;
use std::sync::Arc;

use crate::{fdr_cache::FdrCache, podcast::{Podcast, PodcastNumber}};

pub struct SonicInstance {
    search_channel: SearchChannel,
    ingest_channel: IngestChannel,
    podcast_cache: Arc<FdrCache>
}

impl SonicInstance {
    pub fn new(address: &str, password: &str, podcast_cache: Arc<FdrCache>) -> Self {
        Self {
            search_channel: SearchChannel::start(address, password).unwrap(),
            ingest_channel: IngestChannel::start(address, password).unwrap(),
            podcast_cache
        }
    }

    pub fn search(&self, query: &str) -> Vec<&Arc<Podcast>> {
        self.search_channel.query("fdr", "default", query).unwrap().into_iter().filter_map(|podcast_num| self.podcast_cache.get_podcast(&PodcastNumber::new(Number::from_f64(podcast_num.parse().unwrap()).unwrap()))).collect()
    }

    pub fn suggest(&self, query: &str) -> Vec<String> {
        self.search_channel.suggest_with_limit("fdr", "default", query, 5).unwrap()
    }

    fn ingest(&self, podcast: &Podcast) {
        match self.ingest_channel.push("fdr", "default", &podcast.get_podcast_number().to_string(), podcast.get_title()) {
            Ok(_) => {},
            Err(err) => { println!("Failed to index podcast {}! Error: {}", podcast.get_podcast_number().to_string(), err); }
        };
    }

    pub fn ingest_all(&self) {
        self.podcast_cache.get_all_podcasts().iter().for_each(|podcast| self.ingest(podcast));
    }
}