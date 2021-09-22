use super::meilisearch::SearchResult;
use crate::podcast::PodcastTag;
use std::sync::{Arc, Mutex};

pub struct SearchCache {
    lru: Arc<Mutex<lru::LruCache<(Option<String>, Vec<PodcastTag>), SearchResult>>>,
}

impl SearchCache {
    pub fn new(cap: usize) -> Self {
        Self {
            lru: Arc::from(Mutex::from(lru::LruCache::new(cap))),
        }
    }

    pub async fn search(
        &self,
        query_or: &Option<String>,
        tags: &[PodcastTag],
        limit_or: Option<usize>,
        mut offset: usize,
        meilisearch_backend: &super::meilisearch::MeilisearchBackend,
    ) -> SearchResult {
        let mut tags_vec = Vec::new();
        tags_vec.extend_from_slice(tags);
        {
            let mut lru = self.lru.lock().unwrap();
            let cached_result_or = lru.get(&(query_or.clone(), tags_vec.clone()));

            if let Some(cached_result) = cached_result_or {
                let mut cached_result_clone = cached_result.clone();
                if offset > 0 {
                    if offset > cached_result_clone.hits.len() {
                        offset = cached_result_clone.hits.len();
                    }
                    cached_result_clone.hits.drain(0..offset);
                }
                if let Some(limit) = limit_or {
                    cached_result_clone.hits.truncate(limit);
                };
                println!("Returning cached hit!");
                return cached_result_clone;
            };
        }

        let result = meilisearch_backend
            .search(query_or, tags, limit_or.unwrap_or(99999999), offset)
            .await;

        if limit_or == None && offset == 0 {
            println!("Caching result.");
            let mut lru = self.lru.lock().unwrap();
            lru.put((query_or.clone(), tags_vec), result.clone());
        }

        return result;
    }
}
