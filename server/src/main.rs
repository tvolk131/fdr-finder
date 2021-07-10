#![allow(clippy::all)] // TODO - Remove this line. Rocket expanded macro code currently generates clippy warnings.
#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod environment;
mod fdr_cache;
mod http;
mod podcast;
mod sonic;

use crate::podcast::{generate_rss_feed, Podcast, PodcastNumber, PodcastTag};
use fdr_cache::FdrCache;
use rocket::{
    http::{ContentType, RawStr, Status},
    Request, Response, State,
};
use serde_json::{json, Map, Value};
use sonic::SonicInstance;
use std::io::Cursor;
use std::sync::Arc;

const HTML_BYTES: &[u8] = include_bytes!("../../client/out/index.html");
const JS_BUNDLE_BYTES: &[u8] = include_bytes!("../../client/out/bundle.js");

fn parse_tag_query_string(tags: Option<String>) -> Vec<PodcastTag> {
    match tags {
        Some(tags) => tags
            .split(",")
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
        .filter(|chunk| !chunk.is_empty())
        .next()
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
    let json = Value::Array(
        podcasts
            .into_iter()
            .map(|podcast| podcast.to_json())
            .collect(),
    );

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json.to_string()))
        .finalize()
}

fn search_podcasts<'a, 'b>(
    query: &Option<&String>,
    tags: Vec<PodcastTag>,
    fdr_cache: &'b State<Arc<FdrCache>>,
    sonic_instance: &'b State<SonicInstance>,
) -> Result<Vec<&'b Arc<Podcast>>, Response<'a>> {
    if query.is_none() && tags.is_empty() {
        return Err(Response::build()
            .status(Status::BadRequest)
            .header(ContentType::Plain)
            .sized_body(Cursor::new(
                "Request url must contain `query` or `tags` parameter.",
            ))
            .finalize());
    }

    if query.is_some() && !tags.is_empty() {
        return Err(Response::build()
            .status(Status::BadRequest)
            .header(ContentType::Plain)
            .sized_body(Cursor::new(
                "Support for simultaneous query AND tag filtering is not yet implemented.",
            ))
            .finalize());
    }

    if query.is_some() {
        return Ok(sonic_instance.search_by_title(query.unwrap()));
    } else {
        return Ok(fdr_cache.get_podcasts_by_tags(tags));
    }
}

#[get("/search/podcasts?<query>&<tags>")]
fn search_podcasts_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    fdr_cache: State<Arc<FdrCache>>,
    sonic_instance: State<SonicInstance>,
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

#[get("/search/podcasts/rss?<query>&<tags>")]
fn search_podcasts_as_rss_feed_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    fdr_cache: State<Arc<FdrCache>>,
    sonic_instance: State<SonicInstance>,
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

#[get("/allTags")]
fn get_all_tags_handler<'a>(fdr_cache: State<Arc<FdrCache>>) -> Response<'a> {
    let tags = fdr_cache.get_all_tags();
    let json_tag_array = Value::Array(
        tags.iter()
            .map(|tag| Value::String((*tag).as_ref().to_string()))
            .collect(),
    );

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json_tag_array.to_string()))
        .finalize()
}

#[get("/filteredTagsWithCounts?<tags>")]
fn get_filtered_tags_with_counts_handler<'a>(
    tags: Option<String>,
    fdr_cache: State<Arc<FdrCache>>,
) -> Response<'a> {
    let parsed_tags = parse_tag_query_string(tags);

    let filtered_tags = fdr_cache.get_filtered_tags_with_podcast_counts(parsed_tags);

    let json_tag_array: Value = filtered_tags
        .into_iter()
        .map(|(tag, count)| {
            let mut map = Map::new();
            map.insert(
                "tag".to_string(),
                Value::String((*tag).as_ref().to_string()),
            );
            map.insert("count".to_string(), json!(count));
            Value::Object(map)
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
    let env_vars = environment::EnvironmentVariables::new();
    println!("Fetching podcasts and building cache...");
    let fdr_cache = Arc::from(FdrCache::new().await.unwrap());
    println!("Podcast cache successfully built!");
    println!("Connecting to Sonic...");
    let sonic_instance = SonicInstance::new(
        env_vars.get_sonic_uri().to_string(),
        env_vars.get_sonic_password().to_string(),
        fdr_cache.clone(),
    );
    println!("Ingesting Sonic search index...");
    sonic_instance.ingest_all();
    println!("Search index is complete!");
    println!("Starting server...");
    rocket::ignite()
        .manage(fdr_cache)
        .manage(sonic_instance)
        .register(catchers![not_found_handler])
        .mount(
            "/api",
            routes![
                get_podcast_handler,
                get_all_podcasts_handler,
                search_podcasts_handler,
                search_podcasts_as_rss_feed_handler,
                get_all_tags_handler,
                get_filtered_tags_with_counts_handler
            ],
        )
        .launch();
}
