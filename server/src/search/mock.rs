use crate::{mock::create_mock_podcast, podcast::Podcast};

pub fn generate_mock_search_results() -> Vec<Podcast> {
    let mut mock_podcasts = Vec::new();
    for i in 1..20 {
        mock_podcasts.push(create_mock_podcast(i));
    }
    mock_podcasts
}