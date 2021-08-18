use crate::{mock::create_mock_podcast, podcast::Podcast};
use std::sync::Arc;

pub struct MockBackend {
    mock_podcasts: Vec<Arc<Podcast>>,
}

impl MockBackend {
    pub fn search_by_title(&self, _query: &str) -> Vec<&Arc<Podcast>> {
        self.mock_podcasts.iter().collect()
    }

    pub fn suggest_by_title(&self, _query: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        for i in 1..5 {
            suggestions.push(format!("Suggestion #{}", i));
        }
        suggestions
    }
}

impl Default for MockBackend {
    fn default() -> Self {
        let mut mock_podcasts = Vec::new();
        for i in 1..20 {
            mock_podcasts.push(Arc::from(create_mock_podcast(i)));
        }

        Self { mock_podcasts }
    }
}
