#![allow(clippy::all)] // TODO - Remove this line. Rocket expanded macro code currently generates clippy warnings.
#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod environment;
mod fdr_cache;
mod http;
mod podcast;
mod sonic;

use crate::podcast::{generate_rss_feed, PodcastNumber};
use fdr_cache::FdrCache;
use rocket::{
    http::{ContentType, RawStr, Status},
    Request, Response, State,
};
use serde_json::Value;
use sonic::SonicInstance;
use std::io::Cursor;
use std::sync::Arc;

const HTML_BYTES: &[u8] = include_bytes!("../../client/out/index.html");
const JS_BUNDLE_BYTES: &[u8] = include_bytes!("../../client/out/bundle.js");

#[catch(404)]
fn not_found<'a>(req: &Request) -> Response<'a> {
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
fn get_podcast<'a>(podcast_num: &RawStr, fdr_cache: State<FdrCache>) -> Response<'a> {
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
fn get_all_podcasts<'a>(fdr_cache: State<FdrCache>) -> Response<'a> {
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

#[get("/search/podcasts?<query>")]
fn search_podcasts<'a>(query: String, sonic_instance: State<SonicInstance>) -> Response<'a> {
    let podcasts = sonic_instance.search_by_title(&query);
    let json = Value::Array(podcasts.iter().map(|podcast| podcast.to_json()).collect());

    Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json.to_string()))
        .finalize()
}

#[get("/search/podcasts/rss?<query>")]
fn search_podcasts_as_rss_feed<'a>(
    query: String,
    sonic_instance: State<SonicInstance>,
) -> Response<'a> {
    let podcasts = sonic_instance.search_by_title(&query);
    let rss = generate_rss_feed(
        &podcasts,
        &format!("Freedomain Custom Feed: {}", query),
        &format!(
            "A generated feed containing all Freedomain podcasts about: {}",
            query
        ),
    );

    Response::build()
        .status(Status::Ok)
        .header(ContentType::XML)
        .sized_body(Cursor::new(rss))
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
        env_vars.get_sonic_uri(),
        env_vars.get_sonic_password(),
        fdr_cache.clone(),
    );
    println!("Ingesting Sonic search index...");
    sonic_instance.ingest_all();
    println!("Search index is complete!");
    println!("Starting server...");
    rocket::ignite()
        .manage(fdr_cache)
        .manage(sonic_instance)
        .register(catchers![not_found])
        .mount(
            "/api",
            routes![
                get_podcast,
                get_all_podcasts,
                search_podcasts,
                search_podcasts_as_rss_feed
            ],
        )
        .launch();
}
