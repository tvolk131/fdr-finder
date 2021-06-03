mod environment;
mod fdr_cache;
mod fdr_database;
mod http;
mod podcast;

use environment::EnvironmentVariables;
use fdr_cache::{FdrCache, PodcastQuery};
use fdr_database::FdrDatabase;
use hyper::service::{make_service_fn, service_fn};
use hyper::{http::Error, Body, Request, Response, Server};
use mongodb::{Client, Database};
use serde_json::Value;
use std::{collections::HashMap, net::SocketAddr};
use std::{str::FromStr, sync::Arc};

use crate::{http::get_all_podcasts, podcast::generate_rss_feed};

const HTML_BYTES: &'static [u8] = include_bytes!("../../client/out/index.html");
const JS_BUNDLE_BYTES: &'static [u8] = include_bytes!("../../client/out/bundle.js");

struct HandlerState {
    database: FdrCache,
}

impl HandlerState {
    async fn new(env_vars: &EnvironmentVariables) -> Self {
        HandlerState {
            database: FdrCache::new(FdrDatabase::new(
                get_mongo_database_or_panic(env_vars).await,
            ))
            .await
            .unwrap(),
        }
    }
}

#[tokio::main]
async fn main() {
    let env_vars = EnvironmentVariables::new();
    let port = 80;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let podcasts = get_all_podcasts().await;
    println!("{:?}", podcasts.first().unwrap());

    let handler_state = Arc::from(HandlerState::new(&env_vars).await);

    let make_svc = make_service_fn(move |_| {
        let handler_state = handler_state.clone();

        async {
            Ok::<_, Error>(service_fn(move |req| {
                handle_request(req, handler_state.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Server started on port {}.", port);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn get_mongo_database_or_panic(env_vars: &EnvironmentVariables) -> Database {
    let mongo_client = match Client::with_uri_str(env_vars.get_mongo_uri()).await {
        Ok(client) => client,
        _ => panic!("Failed to connect to MongoDB."),
    };

    mongo_client.database(&env_vars.get_mongo_database())
}

async fn handle_request(
    req: Request<Body>,
    handler_state: Arc<HandlerState>,
) -> Result<Response<Body>, Error> {
    if req.uri().path().starts_with("/api/") {
        return handle_api_request(req, handler_state).await;
    }

    if req.uri().path().split("/").last().unwrap() == "bundle.js" {
        return Response::builder()
            .header("content-type", "application/javascript; charset=utf-8")
            .body(Body::from(JS_BUNDLE_BYTES.to_vec()));
    }
    Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(Body::from(HTML_BYTES.to_vec()))
}

async fn handle_api_request(
    req: Request<Body>,
    handler_state: Arc<HandlerState>,
) -> Result<Response<Body>, Error> {
    if req.uri().path() == "/api/podcasts/all" {
        let podcasts = handler_state.database.get_all_podcasts();
        let json = Value::Array(
            podcasts
                .into_iter()
                .map(|podcast| podcast.to_json())
                .collect(),
        );
        return Response::builder()
            .header("content-type", "application/json")
            .body(Body::from(json.to_string()));
    } else if req.uri().path() == "/api/podcasts" {
        let url = url::Url::from_str(&format!("http://example.com{}", req.uri())).unwrap();
        let query_pairs = url.query_pairs();
        let mut query_params: HashMap<String, String> = HashMap::new();
        for pair in query_pairs {
            query_params.insert(pair.0.to_string(), pair.1.to_string());
        }
        let filter = match query_params.get("filter") {
            Some(filter) => filter.clone(),
            None => String::from(""),
        };
        let limit = match query_params.get("limit") {
            Some(limit) => limit.parse::<usize>().unwrap(),
            None => 0,
        };
        let skip = match query_params.get("skip") {
            Some(skip) => skip.parse::<usize>().unwrap(),
            None => 0,
        };
        let query = PodcastQuery::new(filter, limit, skip);
        let podcasts = handler_state.database.query_podcasts(query);
        let json = Value::Array(
            podcasts
                .into_iter()
                .map(|podcast| podcast.to_json())
                .collect(),
        );
        return Response::builder()
            .header("content-type", "application/json")
            .body(Body::from(json.to_string()));
    } else if req.uri().path() == "/api/podcasts/rss" {
        let url = url::Url::from_str(&format!("http://example.com{}", req.uri())).unwrap();
        let query_pairs = url.query_pairs();
        let mut query_params: HashMap<String, String> = HashMap::new();
        for pair in query_pairs {
            query_params.insert(pair.0.to_string(), pair.1.to_string());
        }
        let filter = match query_params.get("filter") {
            Some(filter) => filter.clone(),
            None => String::from(""),
        };
        let limit = match query_params.get("limit") {
            Some(limit) => limit.parse::<usize>().unwrap(),
            None => 0,
        };
        let skip = match query_params.get("skip") {
            Some(skip) => skip.parse::<usize>().unwrap(),
            None => 0,
        };
        let query = PodcastQuery::new(filter.clone(), limit, skip);
        let podcasts = handler_state.database.query_podcasts(query);
        let rss = generate_rss_feed(&podcasts, &format!("Freedomain Custom Feed: {}", filter), &format!("A generated feed containing all Freedomain podcasts about: {}", filter));
        return Response::builder()
            .header("content-type", "application/xml")
            .body(Body::from(rss));
    } else if req.uri().path().starts_with("/api/podcasts/") {
        let foo: Vec<&str> = req.uri().path().split("/api/podcasts/").collect();
        let bar = foo.get(1).unwrap();
        let podcast = handler_state
            .database
            .get_podcast(&bar.parse::<serde_json::Number>().unwrap());
        return Response::builder()
            .header("content-type", "application/json")
            .body(Body::from(podcast.unwrap().to_json().to_string()));
    }

    Response::builder()
        .status(404)
        .header("content-type", "text/plain")
        .body(Body::from("API endpoint not found"))
}
