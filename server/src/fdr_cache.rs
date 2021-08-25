use crate::http::get_all_podcasts;
use crate::podcast::{Podcast, PodcastNumber};
use std::collections::BTreeMap;
use std::{cmp::Ordering, error::Error};

use crate::mock::create_mock_podcast;

pub struct FdrCache {
    // TODO - Since `podcasts_by_num` is a BTreeMap, it should already be sorted by the key, so I don't think we need `num_sorted_podcast_list`.
    num_sorted_podcast_list: Vec<Podcast>,
    podcasts_by_num: BTreeMap<PodcastNumber, Podcast>,
}

impl FdrCache {
    pub async fn new_with_prod_podcasts() -> Result<Self, Box<dyn Error>> {
        let mut all_podcasts: Vec<Podcast> = get_all_podcasts().await?.into_iter().collect();
        all_podcasts.sort_by(|a, b| {
            if a.get_podcast_number() > b.get_podcast_number() {
                return Ordering::Greater;
            }
            if a.get_podcast_number() < b.get_podcast_number() {
                return Ordering::Less;
            }
            Ordering::Equal
        });
        all_podcasts.reverse();

        Ok(Self::new(all_podcasts))
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
        for podcast in &podcasts {
            podcasts_by_num.insert(podcast.get_podcast_number().clone(), podcast.clone());
        }
        Self {
            num_sorted_podcast_list: podcasts,
            podcasts_by_num,
        }
    }

    // TODO - See if we can get rid of this method and use get_all_podcasts instead.
    pub fn clone_all_podcasts(&self) -> Vec<Podcast> {
        let mut podcasts = Vec::new();
        for podcast in &self.num_sorted_podcast_list {
            podcasts.push(podcast.clone());
        }
        podcasts
    }

    pub fn get_podcast(&self, num: &PodcastNumber) -> Option<&Podcast> {
        self.podcasts_by_num.get(num)
    }
}
