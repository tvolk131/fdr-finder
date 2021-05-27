use std::sync::Arc;
use std::{
    collections::HashMap,
    ops::Add,
    time::{Duration, SystemTime},
};

use bson::{Bson, Document};
use serde_json::{json, Value};

fn get_int_from_bson_doc(doc: &Document, key: &str) -> Option<i32> {
    match doc.get(key)? {
        Bson::Int32(num) => Some(*num),
        Bson::Int64(num) => Some(*num as i32),
        // TODO - Handle this case. Right now we're ignoring fractional numbered episodes
        // since there's only a handful of them, and it would make this much more challenging.
        // Bson::Double(num) => Some(IntOrDouble::F64(*num)),
        _ => None,
    }
}

pub struct Podcast {
    title: String,
    description: String,
    audio_link: String,
    length_in_seconds: i32,
    podcast_number: i32,
    create_time: i32,
}

impl Podcast {
    pub fn from_doc(doc: &Document) -> Self {
        let foo: HashMap<String, String> = doc
            .get_array("urls")
            .unwrap()
            .into_iter()
            .filter_map(|bson| {
                let doc = bson.as_document()?;
                let url_type = match doc.get_str("urlType") {
                    Ok(s) => s,
                    Err(_) => return None,
                }
                .to_string();
                let value = match doc.get_str("value") {
                    Ok(s) => s,
                    Err(_) => return None,
                }
                .to_string();
                Some((url_type, value))
            })
            .collect();
        let audio_link = foo.get("audio").unwrap().clone();
        Self {
            title: doc.get_str("title").unwrap().to_string(),
            description: doc.get_str("description").unwrap().to_string(),
            audio_link,
            length_in_seconds: get_int_from_bson_doc(&doc, "length").unwrap_or(-1),
            podcast_number: get_int_from_bson_doc(&doc, "num").unwrap_or(-1),
            create_time: get_int_from_bson_doc(&doc, "date").unwrap_or(-1),
        }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "title": self.title,
            "description": self.description,
            "audioLink": self.audio_link,
            "lengthInSeconds": self.length_in_seconds,
            "podcastNumber": self.podcast_number,
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

    pub fn get_podcast_number(&self) -> i32 {
        self.podcast_number
    }
}

pub fn generate_rss_feed(podcasts: &[&Arc<Podcast>]) -> String {
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
            <title>Test RSS Feed Title</title>
            <googleplay:author>Stefan Molyneux</googleplay:author>
            <description>A test RSS feed.</description>
            <language>en-us</language>
            <link>https://freedomain.com/</link>
            {}
        </channel>
        </rss>
    ",
        podcasts_xml.join("")
    )
    .trim()
    .to_string()
}
