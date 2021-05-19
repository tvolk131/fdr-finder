use std::cmp::Ordering;
use std::sync::Arc;

use crate::{fdr_database::FdrDatabase, podcast::Podcast};
use std::collections::HashMap;

pub struct FdrCache {
    fdr_database: FdrDatabase,
    num_sorted_podcast_list: Vec<Arc<Podcast>>,
    podcasts_by_num: HashMap<i32, Arc<Podcast>>
}

impl FdrCache {
    pub async fn new(fdr_database: FdrDatabase) -> mongodb::error::Result<Self> {
        let mut all_podcasts = fdr_database.get_all_podcasts().await?;
        all_podcasts.sort_by(|a, b| {
            if a.get_num() > b.get_num() {
                return Ordering::Greater;
            }
            if a.get_num() < b.get_num() {
                return Ordering::Less;
            }
            Ordering::Equal
        });
        let all_podcasts_rc: Vec<Arc<Podcast>> = all_podcasts.into_iter().map(|podcast| Arc::from(podcast)).collect();
        let mut podcasts_by_num = HashMap::new();
        for podcast in &all_podcasts_rc {
            podcasts_by_num.insert(podcast.get_num().clone(), podcast.clone());
        }
        Ok(FdrCache {
            fdr_database,
            num_sorted_podcast_list: all_podcasts_rc,
            podcasts_by_num
        })
    }

    pub fn get_all_podcasts(&self) -> &[Arc<Podcast>] {
        &self.num_sorted_podcast_list
    }

    pub fn get_podcast(&self, num: i32) -> Option<&Arc<Podcast>> {
        self.podcasts_by_num.get(&num)
    }
}