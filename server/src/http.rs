use crate::podcast::{Podcast, PodcastNumber};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

#[derive(Deserialize)]
struct JsonResponse {
    podcasts: Vec<JsonPodcast>,
}

#[derive(Deserialize)]
struct JsonPodcast {
    date: String,
    description: String,
    title: String,
    urls: HashMap<String, String>,
    length: i32,
    num: Option<serde_json::Number>,
}

fn json_podcast_to_podcast(mut json_podcast: JsonPodcast) -> Podcast {
    Podcast::new(
        json_podcast.title,
        json_podcast.description,
        json_podcast.urls.remove("audio").unwrap(),
        json_podcast.length,
        PodcastNumber::new(
            json_podcast
                .num
                .unwrap_or_else(|| serde_json::Number::from(0)),
        ),
        chrono::DateTime::parse_from_rfc3339(&json_podcast.date)
            .unwrap()
            .timestamp(),
    )
}

async fn get_podcasts_page(page_number: i32) -> Result<Vec<Podcast>, Box<dyn Error>> {
    let response_or = reqwest::get(format!(
        "https://fdrpodcasts.com/api/v2/podcasts/?pageNumber={}",
        page_number
    ))
    .await;

    let data_or = match response_or {
        Ok(response) => response.json().await,
        Err(err) => return Err(Box::from(err)),
    };

    let data: JsonResponse = match data_or {
        Ok(data) => data,
        Err(err) => return Err(Box::from(err)),
    };

    Ok(data
        .podcasts
        .into_iter()
        .map(json_podcast_to_podcast)
        .collect())
}

pub async fn get_all_podcasts() -> Result<Vec<Podcast>, Box<dyn Error>> {
    let mut current_page_number = 0;
    let mut results: Vec<Podcast> = Vec::new();
    loop {
        let mut page_results = get_podcasts_page(current_page_number).await?;
        if page_results.is_empty() {
            break;
        }
        results.append(&mut page_results);
        current_page_number += 1;
    }
    Ok(results)
}
