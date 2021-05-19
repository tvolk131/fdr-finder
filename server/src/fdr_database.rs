use std::cmp::Ordering;

use crate::podcast::Podcast;
use bson::{doc, Document};
use futures_lite::StreamExt;
use mongodb::Database;

pub struct FdrDatabase {
    database: Database,
}

impl FdrDatabase {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    async fn get_podcast(&self, find_doc: Document) -> mongodb::error::Result<Option<Podcast>> {
        // TODO - Add projection doc.
        let doc_or = self
            .database
            .collection("podcasts")
            .find_one(find_doc, None)
            .await?;
        Ok(match doc_or {
            Some(doc) => Some(Podcast::from_doc(&doc)),
            None => None,
        })
    }

    // TODO - Make sure that this works correctly with MongoDB `number` *and* `NumberLong`.
    pub async fn get_podcast_by_num_i32(
        &self,
        num: i32,
    ) -> mongodb::error::Result<Option<Podcast>> {
        self.get_podcast(doc! {"num": num}).await
    }

    // TODO - Make sure that this works correctly with MongoDB `number` *and* `double`.
    pub async fn get_podcast_by_num_f64(
        &self,
        num: f64,
    ) -> mongodb::error::Result<Option<Podcast>> {
        self.get_podcast(doc! {"num": num}).await
    }

    pub async fn get_all_podcasts(&self) -> mongodb::error::Result<Vec<Podcast>> {
        // TODO - Add projection doc.
        let cursor = self
            .database
            .collection("podcasts")
            .find(None, None)
            .await?;
        let docs_or = cursor
            .collect::<Vec<Result<Document, mongodb::error::Error>>>()
            .await;
        let mut podcasts: Vec<Podcast> = docs_or
            .iter()
            .filter_map(|doc_or| match doc_or {
                Ok(doc) => Some(Podcast::from_doc(doc)),
                Err(_) => None,
            })
            .collect();
        podcasts.sort_by(|a, b| {
            if a.get_num() > b.get_num() {
                return Ordering::Greater;
            }
            if a.get_num() < b.get_num() {
                return Ordering::Less;
            }
            Ordering::Equal
        });
        Ok(podcasts)
    }
}
