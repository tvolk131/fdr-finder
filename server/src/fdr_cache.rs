use crate::http::get_all_podcasts;
use crate::podcast::{Podcast, PodcastNumber, PodcastTag};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use std::{cmp::Ordering, error::Error};

pub struct PodcastQuery {
    query: String,
    limit: usize,
    skip: usize,
}

impl PodcastQuery {
    pub fn new(query: String, limit: usize, skip: usize) -> Self {
        Self { query, limit, skip }
    }
}

pub struct FdrCache {
    num_sorted_podcast_list: Vec<Arc<Podcast>>,
    podcasts_by_num: BTreeMap<PodcastNumber, Arc<Podcast>>,
    podcasts_by_tag: HashMap<PodcastTag, HashSet<Arc<Podcast>>>,
}

impl FdrCache {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
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
        let all_podcasts_arc: Vec<Arc<Podcast>> = all_podcasts.into_iter().map(Arc::from).collect();

        let mut podcasts_by_tag: HashMap<PodcastTag, HashSet<Arc<Podcast>>> = HashMap::new();
        all_podcasts_arc.iter().for_each(|podcast_arc| {
            podcast_arc.get_tags().iter().for_each(|tag| {
                if !podcasts_by_tag.contains_key(tag) {
                    podcasts_by_tag.insert(tag.clone(), HashSet::new());
                }
                // TODO - Find a way around the unwrap on the line below.
                podcasts_by_tag
                    .get_mut(tag)
                    .unwrap()
                    .insert(podcast_arc.clone());
            })
        });

        let mut podcasts_by_num = BTreeMap::new();
        for podcast in &all_podcasts_arc {
            podcasts_by_num.insert(podcast.get_podcast_number().clone(), podcast.clone());
        }
        Ok(FdrCache {
            num_sorted_podcast_list: all_podcasts_arc,
            podcasts_by_num,
            podcasts_by_tag,
        })
    }

    pub fn query_podcasts(&self, query: PodcastQuery) -> Vec<&Arc<Podcast>> {
        self.num_sorted_podcast_list
            .iter()
            .filter(|podcast| {
                podcast
                    .get_title()
                    .to_lowercase()
                    .contains(&query.query.to_lowercase())
            })
            .skip(query.skip)
            .take(query.limit)
            .collect()
    }

    pub fn get_all_podcasts(&self) -> &[Arc<Podcast>] {
        &self.num_sorted_podcast_list
    }

    // TODO - Unit test this function. There's some complex logic in it, so it might be buggy.
    pub fn get_podcasts_by_tags(&self, tags: &mut Vec<PodcastTag>) -> Vec<&Podcast> {
        // Before slicing for each tag, let's make sure that each tag has at least one valid podcast.
        // If any tag has no podcasts, then we can short-circuit and return an empty vector.
        for tag in tags.iter() {
            match self.podcasts_by_tag.get(tag) {
                Some(tag_podcasts) => {
                    if tag_podcasts.is_empty() {
                        return Vec::new();
                    }
                }
                None => return Vec::new(),
            };
        }

        let first_tag_podcasts_or = match tags.pop() {
            Some(first_tag) => self.podcasts_by_tag.get(&first_tag),
            None => return Vec::new(),
        };
        let mut podcasts: HashSet<Arc<Podcast>> = match first_tag_podcasts_or {
            Some(first_tag_podcasts) => first_tag_podcasts.clone(),
            None => return Vec::new(),
        };
        for tag in tags {
            let tag_podcasts = match self.podcasts_by_tag.get(tag) {
                Some(tag_podcasts) => tag_podcasts,
                None => return Vec::new(),
            };
            let mut podcasts_to_remove = Vec::new();
            for podcast in podcasts.iter() {
                if !tag_podcasts.contains(podcast) {
                    podcasts_to_remove.push(podcast.clone());
                }
            }
            for podcast_to_remove in podcasts_to_remove {
                podcasts.remove(&podcast_to_remove);
            }
        }
        Vec::new()
    }

    pub fn get_podcast(&self, num: &PodcastNumber) -> Option<&Arc<Podcast>> {
        self.podcasts_by_num.get(num)
    }
}
