mod environment;
mod podcast;

use std::net::SocketAddr;
use hyper::{http::Error, Body, Request, Response, Server};
use podcast::Podcast;
use std::sync::Arc;
use hyper::service::{make_service_fn, service_fn};
use environment::EnvironmentVariables;
use mongodb::{Database, Client, options::FindOptions};
use bson::Document;
use futures_lite::StreamExt;
use serde_json::{Number, Value};

const HTML_BYTES: &'static [u8] = include_bytes!("../../client/out/index.html");
const JS_BUNDLE_BYTES: &'static [u8] = include_bytes!("../../client/out/bundle.js");

struct HandlerState {
    database: Database
}

impl HandlerState {
    async fn new(env_vars: &EnvironmentVariables) -> Self {
        HandlerState {
            database: get_mongo_database_or_panic(env_vars).await
        }
    }
}

#[tokio::main]
async fn main() {
    let env_vars = EnvironmentVariables::new();
    let port = 3000;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

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

async fn handle_request(req: Request<Body>, handler_state: Arc<HandlerState>) -> Result<Response<Body>, Error> {
    if req.uri().path().starts_with("/api/") {
        return handle_api_request(req, handler_state).await;
    }

    if req.uri().path().split("/").last().unwrap() == "bundle.js" {
        return Response::builder().header("content-type", "application/javascript; charset=utf-8").body(Body::from(JS_BUNDLE_BYTES.to_vec()))
    }
    Response::builder().header("content-type", "text/html; charset=utf-8").body(Body::from(HTML_BYTES.to_vec()))
}

async fn handle_api_request(req: Request<Body>, handler_state: Arc<HandlerState>) -> Result<Response<Body>, Error> {
    if req.uri().path() == "/api/test" {
        let find_options = FindOptions::builder()
            // .limit(100)
            .build();
        let cursor = handler_state.database.collection("podcasts").find(None, find_options).await.unwrap();
        let docs_or = cursor
            .collect::<Vec<Result<Document, mongodb::error::Error>>>()
            .await;
        let docs: Vec<Document> = docs_or.into_iter().filter_map(|doc_or| {
            match doc_or {
                Ok(doc) => Some(doc),
                Err(_) => None
            }
        }).collect();
        Value::Number(Number::from(1));
        let json_array: Vec<Value> = docs.into_iter().map(|doc: Document| {
            Podcast::from_doc(&doc).to_json()
        }).collect();
        let f = Value::Array(json_array);
        return Response::builder().header("content-type", "application/json").body(Body::from(f.to_string()))
    }

    Response::builder().status(404).header("content-type", "text/plain").body(Body::from("API endpoint not found"))
}