use crate::http::get_all_podcasts;
use crate::podcast::{Podcast, PodcastNumber};
use std::collections::BTreeMap;
use std::error::Error;

use crate::mock::create_mock_podcast;

pub struct FdrCache {
    podcasts_by_num: BTreeMap<PodcastNumber, Podcast>,
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
        let mut podcasts_by_num = BTreeMap::new();
        for podcast in podcasts {
            podcasts_by_num.insert(podcast.get_podcast_number().clone(), podcast);
        }
        Self { podcasts_by_num }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Podcast> {
        self.podcasts_by_num.values()
    }

    pub fn get_podcast(&self, num: &PodcastNumber) -> Option<&Podcast> {
        self.podcasts_by_num.get(num)
    }
}
