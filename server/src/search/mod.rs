use crate::podcast::Podcast;
use std::sync::Arc;

pub mod mock;
pub mod sonic;

pub trait SearchBackend {
    fn search_by_title(&self, query: &str) -> Vec<&Arc<Podcast>>;
    fn suggest_by_title(&self, query: &str) -> Vec<String>;
}

pub trait IngestableBackend {
    fn ingest_all(&self);
}
