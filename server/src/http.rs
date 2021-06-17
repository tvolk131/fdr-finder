use crate::podcast::{Podcast, PodcastNumber};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct JsonResponse {
    podcasts: Vec<JsonPodcast>,
}

#[derive(Deserialize)]
struct JsonUrl {
    #[serde(rename = "urlType")]
    url_type: String,
    value: Option<String>,
}

#[derive(Deserialize)]
struct JsonPodcast {
    date: String,
    description: String,
    title: String,
    urls: Vec<JsonUrl>,
    length: i32,
    num: Option<serde_json::Number>,
}

fn json_podcast_to_podcast(json_podcast: JsonPodcast) -> Podcast {
    let mut audio_links: HashMap<String, String> = json_podcast
        .urls
        .into_iter()
        .map(|url| (url.url_type, url.value.unwrap_or_default()))
        .collect();
    Podcast::new(
        json_podcast.title,
        json_podcast.description,
        audio_links.remove("audio").unwrap(),
        json_podcast.length,
        PodcastNumber::new(json_podcast.num.unwrap_or(serde_json::Number::from(0))),
        chrono::DateTime::parse_from_rfc2822(&json_podcast.date).unwrap().timestamp(),
    )
}

pub async fn get_all_podcasts() -> Result<Vec<Podcast>, String> {
    println!("Getting all podcasts!");
    let response_or = reqwest::get(
        "https://fdrpodcasts.com/api/v2/podcasts/",
    ).await;
    println!("1");

    let data_or = match response_or {
        Ok(response) => {
            response.json().await
        },
        Err(err) => {
            return Err("Uh oh...".to_string())
        }
    };
    println!("2");

    let data: JsonResponse = match data_or {
        Ok(data) => data,
        Err(err) => return Err(err.to_string())
    };

    println!("Got all podcasts!");
    Ok(data
        .podcasts
        .into_iter()
        .map(|json_podcast| json_podcast_to_podcast(json_podcast))
        .collect())
}
