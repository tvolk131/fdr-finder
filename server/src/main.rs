#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod environment;
mod fdr_cache;
mod http;
mod mock;
mod podcast;
mod sonic;

use crate::podcast::{generate_rss_feed, Podcast, PodcastNumber, PodcastTag};
use environment::{EnvironmentVariables, ServerMode};
use fdr_cache::FdrCache;
use rocket::{
    http::{ContentType, RawStr, Status},
    Request, Response, State,
};
use serde_json::{json, Map, Value};
use sonic::{MockSearchBackend, SearchBackend, SonicInstance};
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

fn search_podcasts<'a, 'b>(
    query_or: &Option<&String>,
    tags: Vec<PodcastTag>,
    fdr_cache: &'b State<Arc<FdrCache>>,
    sonic_instance: &'b State<Box<dyn SearchBackend + Send + Sync>>,
) -> Result<Vec<&'b Arc<Podcast>>, Response<'a>> {
    match query_or {
        Some(query) => {
            let query_results = sonic_instance.search_by_title(query);
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
    sonic_instance: State<Box<dyn SearchBackend + Send + Sync>>,
) -> Response<'a> {
    let podcasts = match search_podcasts(
        &query.as_ref(),
        parse_tag_query_string(tags),
        &fdr_cache,
        &sonic_instance,
    ) {
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
    sonic_instance: State<Box<dyn SearchBackend + Send + Sync>>,
) -> Response<'a> {
    let autocomplete_suggestions = match query {
        Some(query) => sonic_instance.suggest_by_title(&query),
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
    sonic_instance: State<Box<dyn SearchBackend + Send + Sync>>,
) -> Response<'a> {
    let podcasts = match search_podcasts(
        &query.as_ref(),
        parse_tag_query_string(tags),
        &fdr_cache,
        &sonic_instance,
    ) {
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
    sonic_instance: State<Box<dyn SearchBackend + Send + Sync>>,
) -> Response<'a> {
    let parsed_tags = parse_tag_query_string(tags);

    let exclusive_podcasts_or =
        query.map(|query| sonic_instance.search_by_title(&query).into_iter().collect());

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

    let search_backend: Box<dyn SearchBackend + Send + Sync> = match server_mode {
        ServerMode::Prod => {
            let sonic_instance = SonicInstance::new(
                env_vars.get_sonic_uri().to_string(),
                env_vars.get_sonic_password().to_string(),
                fdr_cache.clone(),
            );

            println!("Ingesting Sonic search index...");
            sonic_instance.ingest_all();
            println!("Done.");
            Box::from(sonic_instance)
        }
        ServerMode::Mock => Box::from(MockSearchBackend::default()),
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
