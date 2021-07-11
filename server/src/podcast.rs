use std::cmp::Ordering;
use std::sync::Arc;
use std::{
    ops::Add,
    time::{Duration, SystemTime},
};

use serde_json::{json, Number, Value};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PodcastNumber {
    num: Number,
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

    pub fn to_string(&self) -> String {
        self.num.to_string()
    }
}

#[derive(Debug)]
pub struct Podcast {
    title: String,
    description: String,
    audio_link: String,
    length_in_seconds: i32,
    podcast_number: PodcastNumber,
    create_time: i64,
}

impl Podcast {
    pub fn new(
        title: String,
        description: String,
        audio_link: String,
        length_in_seconds: i32,
        podcast_number: PodcastNumber,
        create_time: i64,
    ) -> Self {
        Self {
            title,
            description,
            audio_link,
            length_in_seconds,
            podcast_number,
            create_time,
        }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "title": self.title,
            "description": self.description,
            "audioLink": self.audio_link,
            "lengthInSeconds": self.length_in_seconds,
            "podcastNumber": self.podcast_number.num,
            "createTime": self.create_time
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
}

pub fn generate_rss_feed(
    podcasts: &[&Arc<Podcast>],
    feed_title: &str,
    feed_description: &str,
) -> String {
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
