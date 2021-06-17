use crate::{
    http::get_all_podcasts,
    podcast::{Podcast, PodcastNumber},
};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct PodcastQuery {
    filter: String,
    limit: usize,
    skip: usize,
}

impl PodcastQuery {
    pub fn new(filter: String, limit: usize, skip: usize) -> Self {
        Self {
            filter,
            limit,
            skip,
        }
    }
}

pub struct FdrCache {
    num_sorted_podcast_list: Vec<Arc<Podcast>>,
    podcasts_by_num: BTreeMap<PodcastNumber, Arc<Podcast>>,
}

impl FdrCache {
    pub async fn new() -> Result<Self, String> {
        let mut all_podcasts = get_all_podcasts().await?;
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
        let all_podcasts_rc: Vec<Arc<Podcast>> = all_podcasts
            .into_iter()
            .map(|podcast| Arc::from(podcast))
            .collect();
        let mut podcasts_by_num = BTreeMap::new();
        for podcast in &all_podcasts_rc {
            podcasts_by_num.insert(podcast.get_podcast_number().clone(), podcast.clone());
        }
        Ok(FdrCache {
            num_sorted_podcast_list: all_podcasts_rc,
            podcasts_by_num,
        })
    }

    pub fn get_all_podcasts(&self) -> &[Arc<Podcast>] {
        &self.num_sorted_podcast_list
    }

    pub fn query_podcasts(&self, query: PodcastQuery) -> Vec<&Arc<Podcast>> {
        self.num_sorted_podcast_list
            .iter()
            .filter(|podcast| {
                podcast
                    .get_title()
                    .to_lowercase()
                    .contains(&query.filter.to_lowercase())
            })
            .skip(query.skip)
            .take(query.limit)
            .collect()
    }

    pub fn get_podcast(&self, num: &PodcastNumber) -> Option<&Arc<Podcast>> {
        self.podcasts_by_num.get(num)
    }
}
