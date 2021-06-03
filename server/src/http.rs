use serde::Deserialize;
use crate::podcast::{Podcast};
use std::collections::HashMap;

#[derive(Deserialize)]
struct JsonResponse {
    result: JsonResult
}

#[derive(Deserialize)]
struct JsonResult {
    podcasts: Vec<JsonPodcast>
}

#[derive(Deserialize)]
struct JsonUrl {
    urlType: String,
    value: String
}

#[derive(Deserialize)]
struct JsonPodcast {
    date: i32,
    description: String,
    title: String,
    urls: Vec<JsonUrl>,
    length: i32,
    num: serde_json::Number
}

fn json_podcast_to_podcast(json_podcast: JsonPodcast) -> Podcast {
    let mut audio_links: HashMap<String, String> = json_podcast.urls.into_iter().map(|url| (url.urlType, url.value)).collect();
    Podcast::new(
        json_podcast.title,
        json_podcast.description,
        audio_links.remove("audio").unwrap(),
        json_podcast.length,
        json_podcast.num,
        json_podcast.date
    )
}

pub async fn get_all_podcasts() -> Vec<Podcast> {
    let data: JsonResponse = reqwest::get("http://fdrpodcasts.com/api/?method=ListPodcastFeedItems&feedID=55bd7d968ead0e08688b90d5").await.unwrap().json().await.unwrap();
    data.result.podcasts.into_iter().map(|json_podcast| json_podcast_to_podcast(json_podcast)).collect()
}