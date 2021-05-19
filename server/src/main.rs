mod environment;

use std::net::SocketAddr;
use hyper::{http::Error, Body, Request, Response, Server};
use std::sync::Arc;
use hyper::service::{make_service_fn, service_fn};

const HTML_BYTES: &'static [u8] = include_bytes!("../../client/out/index.html");
const JS_BUNDLE_BYTES: &'static [u8] = include_bytes!("../../client/out/bundle.js");

struct HandlerState {}

impl HandlerState {
    fn new() -> Self {
        HandlerState {}
    }
}

#[tokio::main]
async fn main() {
    let port = 3000;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let handler_state = Arc::from(HandlerState::new());

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
    if req.uri().path().split("/").last().unwrap() == "bundle.js" {
        return Ok(Response::builder().header("content-type", "application/javascript; charset=utf-8").body(Body::from(JS_BUNDLE_BYTES.to_vec())).unwrap())
    }
    Ok(Response::builder().header("content-type", "text/html; charset=utf-8").body(Body::from(HTML_BYTES.to_vec())).unwrap())
}