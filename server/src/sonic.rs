use serde_json::Number;
use sonic_channel::*;
use std::sync::Arc;

use crate::{
    fdr_cache::FdrCache,
    podcast::{Podcast, PodcastNumber},
};

const FDR_COLLECTION: &'static str = "fdr";
const FDR_TITLE_BUCKET: &'static str = "title";

pub struct SonicInstance {
    search_channel: SearchChannel,
    ingest_channel: IngestChannel,
    podcast_cache: Arc<FdrCache>,
}

impl SonicInstance {
    pub fn new(address: &str, password: &str, podcast_cache: Arc<FdrCache>) -> Self {
        Self {
            search_channel: SearchChannel::start(address, password).unwrap(),
            ingest_channel: IngestChannel::start(address, password).unwrap(),
            podcast_cache,
        }
    }

    pub fn search_by_title(&self, query: &str) -> Vec<&Arc<Podcast>> {
        self.search_channel
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

    pub fn suggest_by_title(&self, query: &str) -> Vec<String> {
        self.search_channel
            .suggest_with_limit(FDR_COLLECTION, FDR_TITLE_BUCKET, query, 5)
            .unwrap()
    }

    fn ingest(&self, podcast: &Podcast) {
        match self.ingest_channel.push(
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
        self.podcast_cache
            .get_all_podcasts()
            .iter()
            .for_each(|podcast| self.ingest(podcast));
    }
}
