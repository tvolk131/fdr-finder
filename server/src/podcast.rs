use meilisearch_sdk::document::Document;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::{
    ops::Add,
    time::{Duration, SystemTime},
};

use serde_json::{json, Number, Value};
use sha2::Digest;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PodcastTag(String);

impl PodcastTag {
    pub fn new(tag: String) -> Self {
        Self(tag)
    }

    pub fn clone_to_string(&self) -> String {
        self.0.to_string()
    }
}

// TODO - Replace the tags HashSet with a Vec so that we can derive PartialEq and Hash.
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct PodcastNumber {
    num: Number,
}

impl PartialEq for PodcastNumber {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Hash for PodcastNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl PartialOrd for PodcastNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PodcastNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.num.as_f64() > other.num.as_f64() {
            return Ordering::Greater;
        }
        if self.num.as_f64() < other.num.as_f64() {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

impl PodcastNumber {
    pub fn new(num: Number) -> Self {
        Self { num }
    }
}

impl std::fmt::Display for PodcastNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.num.fmt(f)
    }
}

// TODO - Replace the tags HashSet with a Vec so that we can derive PartialEq and Hash.
#[derive(Clone, Eq, Debug, Serialize, Deserialize)]
pub struct Podcast {
    title: String,
    description: String,
    audio_link: String,
    length_in_seconds: i32,
    podcast_number: PodcastNumber,
    podcast_number_hash: String, // Used as the primary key by Meilisearch, since it contains only alphanumeric characters.
    create_time: i64,
    tags: HashSet<PodcastTag>,
}

impl Document for Podcast {
    type UIDType = PodcastNumber;
    fn get_uid(&self) -> &Self::UIDType {
        &self.podcast_number
    }
}

impl PartialEq for Podcast {
    fn eq(&self, other: &Self) -> bool {
        self.podcast_number == other.podcast_number
    }
}

impl Hash for Podcast {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.podcast_number.hash(state);
    }
}

impl Podcast {
    pub fn new(
        title: String,
        description: String,
        audio_link: String,
        length_in_seconds: i32,
        podcast_number: PodcastNumber,
        create_time: i64,
        tags: HashSet<PodcastTag>,
    ) -> Self {
        let mut hasher = sha2::Sha256::new();
        hasher.update(podcast_number.to_string());
        let podcast_number_hash = hex::encode(hasher.finalize());
        Self {
            title,
            description,
            audio_link,
            length_in_seconds,
            podcast_number,
            podcast_number_hash,
            create_time,
            tags,
        }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "title": self.title,
            "description": self.description,
            "audioLink": self.audio_link,
            "lengthInSeconds": self.length_in_seconds,
            "podcastNumber": self.podcast_number.num,
            "createTime": self.create_time,
            "tags": self.tags.iter().collect::<Vec<&PodcastTag>>()
        })
    }

    fn to_rss_xml(&self) -> String {
        let chrono_date: chrono::DateTime<chrono::Utc> = SystemTime::UNIX_EPOCH
            .add(Duration::from_secs(self.create_time as u64))
            .into();
        format!(
            "
            <title>{}</title>
            <description>{}</description>
            <pubDate>{}</pubDate>
            <enclosure url=\"{}\" type=\"audio/mpeg\" length=\"{}\"/>
        ",
            self.title,
            self.description,
            chrono_date.to_rfc2822(),
            self.audio_link,
            self.length_in_seconds
        )
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_podcast_number(&self) -> &PodcastNumber {
        &self.podcast_number
    }

    pub fn get_tags(&self) -> &HashSet<PodcastTag> {
        &self.tags
    }
}

pub fn generate_rss_feed(podcasts: &[Podcast], feed_title: &str, feed_description: &str) -> String {
    let podcasts_xml: Vec<String> = podcasts
        .iter()
        .map(|podcast| format!("<item>{}</item>", podcast.to_rss_xml()))
        .collect();
    format!(
        "
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>
        <rss version=\"2.0\"
            xmlns:googleplay=\"http://www.google.com/schemas/play-podcasts/1.0\"
            xmlns:itunes=\"http://www.itunes.com/dtds/podcast-1.0.dtd\">
        <channel>
            <title>{}</title>
            <googleplay:author>Stefan Molyneux</googleplay:author>
            <description>{}</description>
            <language>en-us</language>
            <link>https://freedomain.com/</link>
            {}
        </channel>
        </rss>
    ",
        feed_title,
        feed_description,
        podcasts_xml.join("")
    )
    .trim()
    .to_string()
}
