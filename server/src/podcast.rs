use bson::{Bson, Document};
use serde_json::{json, Number, Value};

fn get_json_number_from_bson_doc(doc: &Document, key: &str) -> Option<Number> {
    match doc.get(key)? {
        Bson::Int32(num) => Some(Number::from(*num)),
        Bson::Int64(num) => Some(Number::from(*num)),
        Bson::Double(num) => Number::from_f64(*num),
        _ => None,
    }
}

pub struct Podcast {
    title: String,
    description: String,
    length: Number,
    num: Number,
}

impl Podcast {
    pub fn from_doc(doc: &Document) -> Self {
        Self {
            title: doc.get_str("title").unwrap().to_string(),
            description: doc.get_str("description").unwrap().to_string(),
            length: get_json_number_from_bson_doc(&doc, "length").unwrap_or(Number::from(-1)),
            num: get_json_number_from_bson_doc(&doc, "num").unwrap_or(Number::from(-1)),
        }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "title": self.title,
            "description": self.description,
            "length": self.length,
            "num": self.num
        })
    }
    pub fn get_num(&self) -> &Number {
        &self.num
    }
}
