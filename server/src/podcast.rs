use meilisearch_sdk::document::Document;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::{
    ops::Add,
    time::{Duration, SystemTime},
};

use serde_json::Number;
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

    pub fn to_string(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq)]
pub struct PodcastNumber {
    num: Number,
}

// We're manually implementing Serialize so that a podcast number
// is serialized to a number rather than an object containing a number value.
impl Serialize for PodcastNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.num.serialize(serializer)
    }
}

// We're manually implementing Deserialize so that a podcast number
// is deserialized from a number rather than an object containing a number value.
impl<'de> Deserialize<'de> for PodcastNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PodcastNumber::new(Number::deserialize(deserializer)?))
    }
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
    // TODO - I believe this behavior is undefined if either of the numbers is NOT representable as an f64. Let's handle this case.
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
#[serde(rename_all = "camelCase")]
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

// TODO - Abstract this into a procedural macro along with all other Responder impl blocks in other structs.
impl<'r> rocket::response::Responder<'r, 'static> for Podcast {
    fn respond_to(
        self,
        _request: &'r rocket::request::Request,
    ) -> Result<rocket::response::Response<'static>, rocket::http::Status> {
        let json_string = serde_json::json!(self).to_string();
        rocket::Response::build()
            .header(rocket::http::ContentType::JSON)
            .sized_body(json_string.len(), std::io::Cursor::new(json_string))
            .ok()
    }
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

    fn to_rss_item(&self) -> rss::Item {
        let chrono_date: chrono::DateTime<chrono::Utc> = SystemTime::UNIX_EPOCH
            .add(Duration::from_secs(self.create_time as u64))
            .into();

        rss::ItemBuilder::default()
            .title(self.title.clone())
            .description(self.description.clone())
            .pub_date(chrono_date.to_rfc2822())
            .enclosure(
                rss::EnclosureBuilder::default()
                    .url(self.audio_link.clone())
                    .mime_type("audio/mpeg")
                    .length(format!("{}", self.length_in_seconds))
                    .build(),
            )
            .build()
    }

    pub fn get_podcast_number(&self) -> &PodcastNumber {
        &self.podcast_number
    }

    pub fn get_tags(&self) -> &HashSet<PodcastTag> {
        &self.tags
    }
}

pub struct RssFeed {
    channel: rss::Channel,
}

// TODO - Abstract this into a procedural macro along with all other Responder impl blocks in other structs.
impl<'r> rocket::response::Responder<'r, 'static> for RssFeed {
    fn respond_to(
        self,
        _request: &'r rocket::request::Request,
    ) -> Result<rocket::response::Response<'static>, rocket::http::Status> {
        let xml_string = self.to_string();
        rocket::Response::build()
            .header(rocket::http::ContentType::XML)
            .sized_body(xml_string.len(), std::io::Cursor::new(xml_string))
            .ok()
    }
}

impl RssFeed {
    fn new(channel: rss::Channel) -> Self {
        Self { channel }
    }
}

impl std::string::ToString for RssFeed {
    fn to_string(&self) -> String {
        self.channel.to_string()
    }
}

pub fn generate_rss_feed(
    podcasts: &[Podcast],
    feed_title: &str,
    feed_description: &str,
) -> RssFeed {
    let channel = rss::ChannelBuilder::default()
        .title(feed_title)
        .description(feed_description)
        .language("en-us".to_string())
        .link("https://freedomain.com/")
        .items(
            podcasts
                .iter()
                .map(|podcast| podcast.to_rss_item())
                .collect::<Vec<rss::Item>>(),
        )
        .build();

    RssFeed::new(channel)
}
