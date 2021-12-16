use crate::http::get_all_podcasts;
use crate::podcast::{Podcast, PodcastNumber};
use dashmap::DashMap;
use std::error::Error;
use std::sync::Arc;

use crate::mock::create_mock_podcast;

#[derive(Clone)]
pub struct FdrCache {
    podcasts_by_num: Arc<DashMap<PodcastNumber, Podcast>>,
}

impl FdrCache {
    pub async fn new_with_prod_podcasts() -> Result<Self, Box<dyn Error>> {
        Ok(Self::new(get_all_podcasts().await?.into_iter().collect()))
    }

    pub fn new_with_mock_podcasts() -> Self {
        let mut podcasts: Vec<Podcast> = Vec::new();

        for i in 1..1000 {
            podcasts.push(create_mock_podcast(i));
        }

        Self::new(podcasts)
    }

    fn new(podcasts: Vec<Podcast>) -> Self {
        let mut cache = Self {
            podcasts_by_num: Arc::from(DashMap::new()),
        };
        cache.ingest_podcasts(podcasts.into_iter());
        cache
    }

    pub fn ingest_podcasts(&mut self, podcasts: impl Iterator<Item = Podcast>) {
        for podcast in podcasts {
            self.podcasts_by_num
                .insert(podcast.get_podcast_number().clone(), podcast);
        }
    }

    /// Iterator over all Podcasts in the cache.
    ///
    /// **Locking behaviour:** May deadlock if called when holding any sort of reference into the cache.
    pub fn iter(&self) -> impl Iterator<Item = &Podcast> {
        self.podcasts_by_num.iter().map(|entry| entry.value())
    }

    /// Get a immutable reference to a Podcast in the cache.
    ///
    /// **Locking behaviour:** May deadlock if called when holding a mutable reference into the cache.
    pub fn get_podcast(&self, num: &PodcastNumber) -> Option<&Podcast> {
        match self.podcasts_by_num.get(num) {
            Some(entry) => Some(entry.value()),
            None => None,
        }
    }
}
