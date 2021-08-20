#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod environment;
mod fdr_cache;
mod http;
mod mock;
mod podcast;
mod search;

use crate::podcast::{generate_rss_feed, PodcastNumber, PodcastTag};
use environment::{EnvironmentVariables, ServerMode};
use fdr_cache::FdrCache;
use rocket::response::{content, status};
use rocket::{Request, State};
use search::SearchBackend;
use serde_json::{json, Map, Value};

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
) -> Result<content::Json<String>, status::NotFound<String>> {
    let podcast_or = match podcast_num.parse::<serde_json::Number>() {
        Ok(num) => fdr_cache.get_podcast(&PodcastNumber::new(num)),
        Err(_) => None,
    };

    match podcast_or {
        Some(podcast) => Ok(content::Json(podcast.to_json().to_string())),
        None => Err(status::NotFound("Podcast does not exist".to_string())),
    }
}

#[get("/recentPodcasts?<amount>")]
fn get_recent_podcasts_handler(
    amount: Option<usize>,
    fdr_cache: &State<FdrCache>,
) -> content::Json<String> {
    let podcasts = fdr_cache.get_recent_podcasts(amount.unwrap_or(100));
    let json = Value::Array(podcasts.iter().map(|podcast| podcast.to_json()).collect());
    content::Json(json.to_string())
}

#[get("/search/podcasts?<query>&<limit>&<offset>&<tags>")]
async fn search_podcasts_handler<'a>(
    query: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    tags: Option<String>,
    search_backend: &State<SearchBackend>,
) -> Result<content::Json<String>, status::BadRequest<String>> {
    let podcasts = search_backend
        .search(
            &query,
            &parse_tag_query_string(tags),
            limit,
            offset.unwrap_or(0),
        )
        .await;
    let json = Value::Array(podcasts.iter().map(|podcast| podcast.to_json()).collect());

    Ok(content::Json(json.to_string()))
}

#[get("/search/podcasts/rss?<query>&<tags>")]
async fn search_podcasts_as_rss_feed_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    search_backend: &State<SearchBackend>,
) -> Result<content::Xml<String>, status::BadRequest<String>> {
    let podcasts = search_backend
        .search(&query, &parse_tag_query_string(tags), None, 0)
        .await;

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

    Ok(content::Xml(rss))
}

#[get("/filteredTagsWithCounts?<query>&<tags>")]
async fn get_filtered_tags_with_counts_handler<'a>(
    query: Option<String>,
    tags: Option<String>,
    fdr_cache: &State<FdrCache>,
    search_backend: &State<SearchBackend>,
) -> content::Json<String> {
    let parsed_tags = parse_tag_query_string(tags);

    let exclusive_podcasts_or = match query {
        Some(_) => Some(
            search_backend
                .search(&query, &parsed_tags, None, 0)
                .await
                .into_iter()
                .collect(),
        ),
        None => None,
    };

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

    content::Json(json_tag_array.to_string())
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
                .ingest_podcasts_or_panic(&fdr_cache.clone_all_podcasts())
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
                get_recent_podcasts_handler,
                search_podcasts_handler,
                search_podcasts_as_rss_feed_handler,
                get_filtered_tags_with_counts_handler
            ],
        )
}
