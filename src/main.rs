use axum::{
    body::{Body, boxed, StreamBody},
    Extension,
    http::Request,
    response::Response,
    Router,
    routing::get,
};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
// use axum_extra::body::AsyncReadBody;

type Cache = Arc<RwLock<PathBuf>>;

async fn handle_proxy(
    cache: Extension<Cache>,
    req: Request<Body>,
) -> Response {
    let mut path = String::new();
    {
        let cache = cache.read().unwrap();
        path = String::from(cache.to_str().unwrap());
    }

    // uncomment this:
    // let cache = cache.read().unwrap();

    {
        if let Some(object_key) = req.uri().path().rsplitn(2, '/').next() {
             
            let file = tokio::fs::File::open("/tmp/testaroo").await;
            let buf_reader = tokio::io::BufReader::new(file.unwrap());
            let stream = tokio_util::io::ReaderStream::new(buf_reader);
            return Response::builder().body(boxed(StreamBody::new(stream))).unwrap();
        }
    }

    Response::builder().body(boxed("".to_string())).unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:key", get(handle_proxy));

    let addr = "0.0.0.0:8080";
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
