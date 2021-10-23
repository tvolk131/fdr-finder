#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod environment;
mod fdr_cache;
mod http;
mod mock;
mod podcast;
mod search;

use crate::podcast::{generate_rss_feed, Podcast, PodcastNumber, PodcastTag, RssFeed};
use environment::{EnvironmentVariables, ServerMode};
use fdr_cache::FdrCache;
use rocket::response::{content, status};
use rocket::{Request, State};
use search::SearchBackend;
use search::SearchResult;
use serde_json::{json, Map, Value};
use std::collections::HashMap;

const FAVICON_BYTES: &[u8] = include_bytes!("../../client/out/favicon.ico");
const HTML_BYTES: &[u8] = include_bytes!("../../client/out/index.html");
const JS_BUNDLE_BYTES: &[u8] = include_bytes!("../../client/out/bundle.js");

fn parse_tag_query_string(tags: Option<String>) -> Vec<PodcastTag> {
    match tags {
        Some(tags) => tags
            .split(',')
            .map(|tag| PodcastTag::new(tag.trim().to_string()))
            .collect(),
        None => Vec::new(),
    }
}

enum NotFoundResponse {
    Html(status::Custom<content::Html<&'static [u8]>>),
    JavaScript(status::Custom<content::JavaScript<&'static [u8]>>),
    Favicon(Box<status::Custom<content::Custom<&'static [u8]>>>),
    NotFound(status::NotFound<String>),
}

impl<'r> rocket::response::Responder<'r, 'static> for NotFoundResponse {
    fn respond_to(
        self,
        request: &'r Request<'_>,
    ) -> Result<rocket::response::Response<'static>, rocket::http::Status> {
        match self {
            NotFoundResponse::Html(html) => html.respond_to(request),
            NotFoundResponse::JavaScript(javascript) => javascript.respond_to(request),
            NotFoundResponse::Favicon(favicon) => favicon.respond_to(request),
            NotFoundResponse::NotFound(not_found) => not_found.respond_to(request),
        }
    }
}

#[catch(404)]
fn not_found_handler(req: &Request) -> NotFoundResponse {
    let last_chunk = match req.uri().path().split('/').last() {
        Some(raw_str) => raw_str.as_str().to_string(),
        None => "".to_string(),
    };

    if req
        .uri()
        .path()
        .split('/')
        .find(|chunk| !chunk.is_empty())
        .unwrap_or_else(|| "".into())
        == "api"
    {
        NotFoundResponse::NotFound(status::NotFound(format!(
            "404 - API path '{}' does not exist!",
            req.uri().path()
        )))
    } else if last_chunk == "bundle.js" {
        NotFoundResponse::JavaScript(status::Custom(
            rocket::http::Status::Ok,
            content::JavaScript(JS_BUNDLE_BYTES),
        ))
    } else if last_chunk == "favicon.ico" {
        NotFoundResponse::Favicon(Box::from(status::Custom(
            rocket::http::Status::Ok,
            content::Custom(rocket::http::ContentType::Icon, FAVICON_BYTES),
        )))
    } else {
        NotFoundResponse::Html(status::Custom(
            rocket::http::Status::Ok,
            content::Html(HTML_BYTES),
        ))
    }
}

#[get("/podcasts/<podcast_num>")]
fn get_podcast_handler(
    podcast_num: String,
    fdr_cache: &State<FdrCache>,
) -> Result<Podcast, status::NotFound<String>> {
    let podcast_or = match podcast_num.parse::<serde_json::Number>() {
        Ok(num) => fdr_cache.get_podcast(&PodcastNumber::new(num)),
        Err(_) => None,
    };

    match podcast_or {
        Some(podcast) => Ok(podcast.clone()),
        None => Err(status::NotFound("Podcast does not exist".to_string())),
    }
}

#[get("/search/podcasts?<query>&<limit>&<offset>&<tags>")]
async fn search_podcasts_handler<'a>(
    query: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    tags: Option<String>,
    search_backend: &State<SearchBackend>,
) -> SearchResult {
    search_backend
        .search(
            &query,
            &parse_tag_query_string(tags),
            limit,
            offset.unwrap_or(0),
        )
        .await
}

#[get("/search/podcasts/rss?<query>&<tags>")]
async fn search_podcasts_as_rss_feed_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    search_backend: &State<SearchBackend>,
) -> RssFeed {
    let search_result = search_backend
        .search(&query, &parse_tag_query_string(tags), None, 0)
        .await;

    // TODO - Fix RSS feed naming now that we support tag filtering.
    generate_rss_feed(
        search_result.get_hits(),
        &format!(
            "Freedomain Custom Feed: {}",
            query.clone().unwrap_or_default()
        ),
        &format!(
            "A generated feed containing all Freedomain podcasts about: {}",
            query.unwrap_or_default()
        ),
    )
}

#[get("/filteredTagsWithCounts?<query>&<limit>&<offset>&<tags>&<filter>")]
async fn get_filtered_tags_with_counts_handler<'a>(
    query: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    tags: Option<String>,
    filter: Option<String>,
    search_backend: &State<SearchBackend>,
) -> content::Json<String> {
    let parsed_tags = parse_tag_query_string(tags);

    let podcasts: Vec<Podcast> = search_backend
        .search(&query, &parsed_tags, None, 0)
        .await
        .take_hits()
        .into_iter()
        .collect();

    let mut counts_by_tag = HashMap::new();
    for podcast in &podcasts {
        for tag in podcast.get_tags() {
            match counts_by_tag.get_mut(tag) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    counts_by_tag.insert(tag.clone(), 1);
                }
            };
        }
    }

    // Delete tags that have already been selected.
    for tag in &parsed_tags {
        counts_by_tag.remove(tag);
    }

    let mut counts_list: Vec<(PodcastTag, usize)> = counts_by_tag.into_iter().collect();
    if let Some(filter) = filter {
        counts_list = counts_list
            .into_iter()
            .filter(|(tag, _count)| {
                tag.to_string()
                    .to_lowercase()
                    .contains(&filter.to_lowercase())
            })
            .collect();
    }
    counts_list.sort_by(|(tag_one, count_one), (tag_two, count_two)| {
        let count_ordering = count_two.cmp(count_one);
        if count_ordering == std::cmp::Ordering::Equal {
            return tag_one
                .to_string()
                .to_lowercase()
                .cmp(&tag_two.to_string().to_lowercase());
        } else {
            count_ordering
        }
    });
    let pretrimmed_tag_count = counts_list.len();
    if let Some(mut offset) = offset {
        // Over-draining causes a panic.
        // Here we're making sure that we
        // don't drain more items than
        // exist in the array.
        if offset > counts_list.len() {
            offset = counts_list.len();
        }
        counts_list.drain(0..offset);
    }
    if let Some(limit) = limit {
        counts_list.truncate(limit);
    }
    let trimmed_tag_count = counts_list.len();

    let json_tag_array: Value = counts_list
        .into_iter()
        .map(|(tag, count)| {
            let mut obj = Map::new();
            obj.insert("tag".to_string(), Value::String(tag.clone_to_string()));
            obj.insert("count".to_string(), json!(count));
            Value::Object(obj)
        })
        .collect();

    let mut obj = Map::new();
    obj.insert("tags".to_string(), json_tag_array);
    obj.insert(
        "remainingTagCount".to_string(),
        json!(pretrimmed_tag_count - trimmed_tag_count),
    );
    let json_obj = Value::Object(obj);

    content::Json(json_obj.to_string())
}

#[rocket::launch]
async fn rocket() -> _ {
    let env_vars = EnvironmentVariables::default();

    let server_mode = env_vars.get_server_mode();

    match server_mode {
        ServerMode::Prod => {
            println!("Running in production mode.");
        }
        ServerMode::Mock => {
            println!("Running in mock mode.");
        }
    }

    let fdr_cache = match server_mode {
        ServerMode::Prod => {
            println!("Fetching podcasts and building cache...");
            let fdr_cache = FdrCache::new_with_prod_podcasts().await.unwrap();
            println!("Done.");
            fdr_cache
        }
        ServerMode::Mock => {
            println!("Generating mock podcasts...");
            let fdr_cache = FdrCache::new_with_mock_podcasts();
            println!("Done.");
            fdr_cache
        }
    };

    let search_backend: SearchBackend = match server_mode {
        ServerMode::Prod => {
            let search_backend = SearchBackend::new_prod(
                env_vars.get_meilisearch_host().to_string(),
                env_vars.get_meilisearch_api_key().to_string(),
            )
            .await;

            println!("Ingesting search index...");
            search_backend
                .ingest_podcasts_or_panic(fdr_cache.iter())
                .await;
            println!("Done.");
            search_backend
        }
        ServerMode::Mock => SearchBackend::new_mock(),
    };

    println!("Starting server...");
    rocket::build()
        .manage(fdr_cache)
        .manage(search_backend)
        .register("/", catchers![not_found_handler])
        .mount(
            "/api",
            routes![
                get_podcast_handler,
                search_podcasts_handler,
                search_podcasts_as_rss_feed_handler,
                get_filtered_tags_with_counts_handler
            ],
        )
}
