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
    address: String,
    password: String,
    podcast_cache: Arc<FdrCache>,
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

    pub fn search_by_title(&self, query: &str) -> Vec<&Arc<Podcast>> {
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

    pub fn suggest_by_title(&self, query: &str) -> Vec<String> {
        SearchChannel::start(&self.address, &self.password)
            .unwrap()
            .suggest_with_limit(FDR_COLLECTION, FDR_TITLE_BUCKET, query, 5)
            .unwrap()
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
