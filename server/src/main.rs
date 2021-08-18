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
use rocket::{
    http::{ContentType, RawStr, Status},
    Request, Response, State,
};
use search::SearchBackend;
use serde_json::{json, Map, Value};
use std::collections::HashSet;
use std::io::Cursor;
use std::sync::Arc;

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
fn not_found_handler<'a>(req: &Request) -> Response<'a> {
    if req
        .uri()
        .path()
        .split('/')
        .find(|chunk| !chunk.is_empty())
        .unwrap_or_default()
        == "api"
    {
        return Response::build()
            .status(Status::NotFound)
            .header(ContentType::Plain)
            .sized_body(Cursor::new(format!(
                "404 - API path '{}' does not exist!",
                req.uri().path()
            )))
            .finalize();
    } else if req.uri().path().split('/').last().unwrap_or_default() == "bundle.js" {
        return Response::build()
            .status(Status::Ok)
            .header(ContentType::JavaScript)
            .sized_body(Cursor::new(JS_BUNDLE_BYTES))
            .finalize();
    } else {
        return Response::build()
            .status(Status::Ok)
            .header(ContentType::HTML)
            .sized_body(Cursor::new(HTML_BYTES))
            .finalize();
    }
}

#[get("/podcasts/<podcast_num>")]
fn get_podcast_handler<'a>(podcast_num: &RawStr, fdr_cache: State<Arc<FdrCache>>) -> Response<'a> {
    let podcast_or = match podcast_num.parse::<serde_json::Number>() {
        Ok(num) => fdr_cache.get_podcast(&PodcastNumber::new(num)),
        Err(_) => None,
    };

    return match podcast_or {
        Some(podcast) => Response::build()
            .status(Status::Ok)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(podcast.to_json().to_string()))
            .finalize(),
        None => Response::build()
            .status(Status::NotFound)
            .header(ContentType::HTML)
            .sized_body(Cursor::new("Podcast does not exist"))
            .finalize(),
    };
}

#[get("/allPodcasts")]
fn get_all_podcasts_handler<'a>(fdr_cache: State<Arc<FdrCache>>) -> Response<'a> {
    let podcasts = fdr_cache.get_all_podcasts();
    let json = Value::Array(podcasts.iter().map(|podcast| podcast.to_json()).collect());

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json.to_string()))
        .finalize()
}

#[get("/recentPodcasts?<amount>")]
fn get_recent_podcasts_handler<'a>(
    amount: Option<usize>,
    fdr_cache: State<Arc<FdrCache>>,
) -> Response<'a> {
    let podcasts = fdr_cache.get_recent_podcasts(amount.unwrap_or(100));
    let json = Value::Array(podcasts.iter().map(|podcast| podcast.to_json()).collect());

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json.to_string()))
        .finalize()
}

fn get_intersection_of_podcast_lists<'a>(
    list_one: Vec<&'a Arc<Podcast>>,
    list_two: Vec<&'a Arc<Podcast>>,
) -> Vec<&'a Arc<Podcast>> {
    let podcast_set: HashSet<&'a Arc<Podcast>> = list_one.into_iter().collect();
    let mut intersecting_podcasts: Vec<&'a Arc<Podcast>> = Vec::new();
    for podcast in list_two.into_iter() {
        if podcast_set.contains(&podcast) {
            intersecting_podcasts.push(podcast);
        }
    }
    intersecting_podcasts
}

async fn search_podcasts<'a, 'b>(
    query_or: &Option<&String>,
    tags: Vec<PodcastTag>,
    fdr_cache: &'b State<'_, Arc<FdrCache>>,
    search_backend: &'b State<'_, SearchBackend>,
) -> Result<Vec<&'b Arc<Podcast>>, Response<'a>> {
    match query_or {
        Some(query) => {
            let query_results = search_backend.search_by_title(query).await;
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
                Err(Response::build()
                    .status(Status::BadRequest)
                    .header(ContentType::Plain)
                    .sized_body(Cursor::new(
                        "Request url must contain `query` or `tags` parameter.",
                    ))
                    .finalize())
            } else {
                Ok(fdr_cache.get_podcasts_by_tags(tags))
            }
        }
    }
}

#[get("/search/podcasts?<query>&<tags>")]
fn search_podcasts_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    fdr_cache: State<Arc<FdrCache>>,
    search_backend: State<SearchBackend>,
) -> Response<'a> {
    // TODO - Don't block on futures. Find a way to make the Rocket handler async instead.
    let podcasts = match futures::executor::block_on(search_podcasts(
        &query.as_ref(),
        parse_tag_query_string(tags),
        &fdr_cache,
        &search_backend,
    )) {
        Ok(podcasts) => podcasts,
        Err(res) => return res,
    };
    let json = Value::Array(podcasts.iter().map(|podcast| podcast.to_json()).collect());

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json.to_string()))
        .finalize()
}

#[get("/search/podcasts/autocomplete?<query>")]
fn search_podcasts_autocomplete_handler<'a>(
    query: Option<String>,
    search_backend: State<SearchBackend>,
) -> Response<'a> {
    let autocomplete_suggestions = match query {
        // TODO - Don't block on futures. Find a way to make the Rocket handler async instead.
        Some(query) => futures::executor::block_on(search_backend.suggest_by_title(&query)),
        None => Vec::new(),
    };

    let json = Value::Array(
        autocomplete_suggestions
            .into_iter()
            .map(Value::String)
            .collect(),
    );

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json.to_string()))
        .finalize()
}

#[get("/search/podcasts/rss?<query>&<tags>")]
fn search_podcasts_as_rss_feed_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    fdr_cache: State<Arc<FdrCache>>,
    search_backend: State<SearchBackend>,
) -> Response<'a> {
    // TODO - Don't block on futures. Find a way to make the Rocket handler async instead.
    let podcasts = match futures::executor::block_on(search_podcasts(
        &query.as_ref(),
        parse_tag_query_string(tags),
        &fdr_cache,
        &search_backend,
    )) {
        Ok(podcasts) => podcasts,
        Err(res) => return res,
    };
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

    Response::build()
        .status(Status::Ok)
        .header(ContentType::XML)
        .sized_body(Cursor::new(rss))
        .finalize()
}

#[get("/filteredTagsWithCounts?<query>&<tags>")]
fn get_filtered_tags_with_counts_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    fdr_cache: State<Arc<FdrCache>>,
    search_backend: State<SearchBackend>,
) -> Response<'a> {
    let parsed_tags = parse_tag_query_string(tags);

    // TODO - Don't block on futures. Find a way to make the Rocket handler async instead.
    let exclusive_podcasts_or =
        query.map(|query| futures::executor::block_on(search_backend.search_by_title(&query)).into_iter().collect());

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

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json_tag_array.to_string()))
        .finalize()
}

#[tokio::main]
async fn main() {
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

    let fdr_cache = Arc::from(match server_mode {
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
    });

    let search_backend: SearchBackend = match server_mode {
        ServerMode::Prod => {
            let search_backend = SearchBackend::new_prod(
                env_vars.get_sonic_uri().to_string(),
                env_vars.get_sonic_password().to_string(),
                env_vars.get_meilisearch_host().to_string(),
                env_vars.get_meilisearch_api_key().to_string(),
                fdr_cache.clone()
            ).await;

            println!("Ingesting search index...");
            search_backend.ingest_podcasts(&fdr_cache.clone_all_podcasts()).await;
            println!("Done.");
            search_backend
        }
        ServerMode::Mock => SearchBackend::new_mock(),
    };

    println!("Starting server...");
    rocket::ignite()
        .manage(fdr_cache)
        .manage(search_backend)
        .register(catchers![not_found_handler])
        .mount(
            "/api",
            routes![
                get_podcast_handler,
                get_all_podcasts_handler,
                get_recent_podcasts_handler,
                search_podcasts_handler,
                search_podcasts_autocomplete_handler,
                search_podcasts_as_rss_feed_handler,
                get_filtered_tags_with_counts_handler
            ],
        )
        .launch();
}
