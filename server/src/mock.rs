use crate::podcast::{Podcast, PodcastNumber, PodcastTag};
use std::collections::HashSet;

pub fn create_mock_podcast(num: usize) -> Podcast {
    let mut tags = HashSet::new();
    tags.insert(PodcastTag::new(format!("Tag #{}", num % 100)));

    Podcast::new(
        format!("Podcast #{}", num),
        format!("Description of podcast #{}", num),
        format!("http://example.com/podcasts/{}", num),
        num,
        PodcastNumber::new(serde_json::Number::from(num)),
        12341627541915 + (num as i64),
        tags,
    )
}
