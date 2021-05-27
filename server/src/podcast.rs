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
    length_in_seconds: i32,
    podcast_number: i32,
}

impl Podcast {
    pub fn from_doc(doc: &Document) -> Self {
        Self {
            title: doc.get_str("title").unwrap().to_string(),
            description: doc.get_str("description").unwrap().to_string(),
            length_in_seconds: get_int_from_bson_doc(&doc, "length").unwrap_or(-1),
            podcast_number: get_int_from_bson_doc(&doc, "num").unwrap_or(-1),
        }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "title": self.title,
            "description": self.description,
            "lengthInSeconds": self.length_in_seconds,
            "podcastNumber": self.podcast_number
        })
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_podcast_number(&self) -> i32 {
        self.podcast_number
    }
}
