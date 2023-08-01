use redis::{ Commands, RedisResult };
use std::{ collections::HashMap, net::SocketAddr, sync::{ Arc, Mutex } };
use tower_http::trace::{ self, TraceLayer };
use tracing::Level;

use axum::{ routing::{ any, get }, Router };

mod cache;
mod proxer;

pub struct AppState {
    redis_con: redis::Connection,
}

#[tokio::main]
async fn main() -> RedisResult<()> {
    tracing_subscriber::fmt().with_target(false).compact().init();

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con: redis::Connection = client.get_connection()?;
    // let apis: HashMap<String, String> = con.hgetall("apis")?;
    // println!("{:?}", apis);

    let shared_state = Arc::new(Mutex::new(AppState { redis_con: con }));

    // build our application with a single route
    let app = Router::new()
        .route(
            "/hello",
            get(|| async { "Hello, World!" })
        )
        .route("/*path", any(proxer::proxer))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        )
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

    Ok(())
}
