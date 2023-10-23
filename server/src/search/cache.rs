use super::meilisearch::SearchResult;
use crate::podcast::PodcastTag;
use std::sync::{Arc, Mutex};

type SearchLru = lru::LruCache<
    (
        Option<String>,
        Vec<PodcastTag>,
        Option<usize>,
        Option<usize>,
    ),
    SearchResult,
>;

#[derive(Clone)]
pub struct SearchCache {
    lru: Arc<Mutex<SearchLru>>,
}

// TODO - Optimize the locking behavior of this struct, or possibly make it non-blocking.
impl SearchCache {
    pub fn new(cap: usize) -> Self {
        Self {
            lru: Arc::from(Mutex::from(lru::LruCache::new(cap))),
        }
    }

    pub fn reset(&self) {
        self.lru.lock().unwrap().clear();
    }

    // TODO - Find a way to reduce the number of arguments so we can remove this.
    #[allow(clippy::too_many_arguments)]
    pub async fn search(
        &self,
        query_or: &Option<String>,
        tags: &[PodcastTag],
        limit_or: Option<usize>,
        mut offset: usize,
        min_length_seconds: Option<usize>,
        max_length_seconds: Option<usize>,
        meilisearch_backend: &super::meilisearch::MeilisearchBackend,
    ) -> SearchResult {
        let mut tags_vec = Vec::new();
        tags_vec.extend_from_slice(tags);
        {
            let mut lru = self.lru.lock().unwrap();
            let cached_result_or = lru.get(&(
                query_or.clone(),
                tags_vec.clone(),
                min_length_seconds,
                max_length_seconds,
            ));

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
                return cached_result_clone;
            };
        }

        let result = meilisearch_backend
            .search(
                query_or,
                tags,
                limit_or.unwrap_or(99999999),
                offset,
                min_length_seconds,
                max_length_seconds,
            )
            .await;

        if limit_or.is_none() && offset == 0 {
            let mut lru = self.lru.lock().unwrap();
            lru.put(
                (
                    query_or.clone(),
                    tags_vec,
                    min_length_seconds,
                    max_length_seconds,
                ),
                result.clone(),
            );
        }

        result
    }
}
