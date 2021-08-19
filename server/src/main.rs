#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod environment;
mod fdr_cache;
mod http;
mod mock;
mod podcast;
mod search;

use crate::podcast::{generate_rss_feed, Podcast, PodcastNumber, PodcastTag};
use environment::{EnvironmentVariables, ServerMode};
use fdr_cache::FdrCache;
use rocket::{Request, State};
use search::SearchBackend;
use serde_json::{json, Map, Value};
use std::collections::HashSet;

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

#[catch(404)]
fn not_found_handler(
    req: &Request,
) -> Result<
    Result<
        rocket::response::content::Html<&'static [u8]>,
        rocket::response::content::JavaScript<&'static [u8]>,
    >,
    rocket::response::status::NotFound<String>,
> {
    if req
        .uri()
        .path()
        .split('/')
        .find(|chunk| !chunk.is_empty())
        .unwrap_or_else(|| "".into())
        == "api"
    {
        Err(rocket::response::status::NotFound(format!(
            "404 - API path '{}' does not exist!",
            req.uri().path()
        )))
    } else if req.uri().path().split('/').last().unwrap_or_else(|| "".into()) == "bundle.js" {
        Ok(Err(rocket::response::content::JavaScript(JS_BUNDLE_BYTES)))
    } else {
        Ok(Ok(rocket::response::content::Html(HTML_BYTES)))
    }
}

#[get("/podcasts/<podcast_num>")]
fn get_podcast_handler(
    podcast_num: String,
    fdr_cache: &State<FdrCache>,
) -> Result<rocket::response::content::Json<String>, rocket::response::status::NotFound<String>> {
    let podcast_or = match podcast_num.parse::<serde_json::Number>() {
        Ok(num) => fdr_cache.get_podcast(&PodcastNumber::new(num)),
        Err(_) => None,
    };

    match podcast_or {
        Some(podcast) => Ok(rocket::response::content::Json(
            podcast.to_json().to_string(),
        )),
        None => Err(rocket::response::status::NotFound(
            "Podcast does not exist".to_string(),
        )),
    }
}

#[get("/allPodcasts")]
fn get_all_podcasts_handler(
    fdr_cache: &State<FdrCache>,
) -> rocket::response::content::Json<String> {
    let podcasts = fdr_cache.get_all_podcasts();
    let json = Value::Array(podcasts.iter().map(|podcast| podcast.to_json()).collect());
    rocket::response::content::Json(json.to_string())
}

#[get("/recentPodcasts?<amount>")]
fn get_recent_podcasts_handler(
    amount: Option<usize>,
    fdr_cache: &State<FdrCache>,
) -> rocket::response::content::Json<String> {
    let podcasts = fdr_cache.get_recent_podcasts(amount.unwrap_or(100));
    let json = Value::Array(podcasts.iter().map(|podcast| podcast.to_json()).collect());
    rocket::response::content::Json(json.to_string())
}

fn get_intersection_of_podcast_lists(
    list_one: Vec<Podcast>,
    list_two: Vec<Podcast>,
) -> Vec<Podcast> {
    let podcast_set: HashSet<Podcast> = list_one.into_iter().collect();
    let mut intersecting_podcasts: Vec<Podcast> = Vec::new();
    for podcast in list_two.into_iter() {
        if podcast_set.contains(&podcast) {
            intersecting_podcasts.push(podcast);
        }
    }
    intersecting_podcasts
}

fn search_podcasts<'a, 'b>(
    query_or: &Option<&String>,
    limit_or: Option<usize>,
    offset: usize,
    tags: Vec<PodcastTag>,
    fdr_cache: &'b State<FdrCache>,
    search_backend: &'b State<SearchBackend>,
) -> Result<Vec<Podcast>, rocket::response::status::BadRequest<String>> {
    match query_or {
        Some(query) => {
            let query_results = search_backend.search(query, limit_or, offset);
            if tags.is_empty() {
                Ok(query_results)
            } else {
                Ok(get_intersection_of_podcast_lists(
                    query_results,
                    fdr_cache.get_podcasts_by_tags(tags),
                ))
            }
        }
        None => {
            if tags.is_empty() {
                Err(rocket::response::status::BadRequest(Some(
                    "Request url must contain `query` or `tags` parameter.".to_string(),
                )))
            } else {
                Ok(fdr_cache.get_podcasts_by_tags(tags))
            }
        }
    }
}

#[get("/search/podcasts?<query>&<limit>&<offset>&<tags>")]
async fn search_podcasts_handler<'a>(
    query: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    tags: Option<String>,
    fdr_cache: &State<FdrCache>,
    search_backend: &State<SearchBackend>,
) -> Result<rocket::response::content::Json<String>, rocket::response::status::BadRequest<String>> {
    let podcasts = search_podcasts(
        &query.as_ref(),
        limit,
        offset.unwrap_or(0),
        parse_tag_query_string(tags),
        fdr_cache,
        search_backend,
    )?;
    let json = Value::Array(podcasts.iter().map(|podcast| podcast.to_json()).collect());

    Ok(rocket::response::content::Json(json.to_string()))
}

#[get("/search/podcasts/rss?<query>&<tags>")]
async fn search_podcasts_as_rss_feed_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    fdr_cache: &State<FdrCache>,
    search_backend: &State<SearchBackend>,
) -> Result<rocket::response::content::Xml<String>, rocket::response::status::BadRequest<String>> {
    let podcasts = search_podcasts(
        &query.as_ref(),
        None,
        0,
        parse_tag_query_string(tags),
        fdr_cache,
        search_backend,
    )?;

    // TODO - Fix RSS feed naming now that we support tag filtering.
    let rss = generate_rss_feed(
        &podcasts,
        &format!(
            "Freedomain Custom Feed: {}",
            query.clone().unwrap_or_default()
        ),
        &format!(
            "A generated feed containing all Freedomain podcasts about: {}",
            query.unwrap_or_default()
        ),
    );

    Ok(rocket::response::content::Xml(rss))
}

#[get("/filteredTagsWithCounts?<query>&<tags>")]
async fn get_filtered_tags_with_counts_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    fdr_cache: &State<FdrCache>,
    search_backend: &State<SearchBackend>,
) -> rocket::response::content::Json<String> {
    let parsed_tags = parse_tag_query_string(tags);

    let exclusive_podcasts_or = query.map(|query| search_backend.search(&query, None, 0).into_iter().collect());

    let filtered_tags =
        fdr_cache.get_filtered_tags_with_podcast_counts(exclusive_podcasts_or, parsed_tags);

    let json_tag_array: Value = filtered_tags
        .into_iter()
        .map(|(tag, count)| {
            let mut obj = Map::new();
            obj.insert("tag".to_string(), Value::String(tag.clone_to_string()));
            obj.insert("count".to_string(), json!(count));
            Value::Object(obj)
        })
        .collect();

    rocket::response::content::Json(json_tag_array.to_string())
}

// TODO - Remove `block_on` calls in this function, and instead make the function async.
#[rocket::launch]
fn rocket() -> _ {
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
            let fdr_cache =
                futures::executor::block_on(FdrCache::new_with_prod_podcasts()).unwrap();
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
            let search_backend = futures::executor::block_on(SearchBackend::new_prod(
                env_vars.get_meilisearch_host().to_string(),
                env_vars.get_meilisearch_api_key().to_string(),
            ));

            println!("Ingesting search index...");
            futures::executor::block_on(
                search_backend.ingest_podcasts_or_panic(&fdr_cache.clone_all_podcasts()),
            );
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
                get_all_podcasts_handler,
                get_recent_podcasts_handler,
                search_podcasts_handler,
                search_podcasts_as_rss_feed_handler,
                get_filtered_tags_with_counts_handler
            ],
        )
}
