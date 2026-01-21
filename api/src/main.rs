use axum::{
    routing::post,
    middleware,
    Router,
    Extension,
};
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::net::TcpListener;

use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};

use zkcg_verifier::engine::VerifierEngine;
use zkcg_common::state::ProtocolState;
use zkcg_verifier::backend_zkvm::ZkVmBackend;

use api::handler::{
    submit_proof,
    prove,
    demo_prove_handler,
    demo_verify_handler,
    AppState,
};

mod rate_limit;
use rate_limit::RateLimiter;

// 👇 simple request logger middleware
async fn log_requests(
    req: Request<Body>,
    next: Next,
) -> Response {
    println!(
        "[REQUEST] {} {}",
        req.method(),
        req.uri().path()
    );

    let res = next.run(req).await;

    println!(
        "[RESPONSE] status={}",
        res.status()
    );

    res
}

#[tokio::main]
async fn main() {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("[BOOT] starting ZKCG API");

    let engine = VerifierEngine::new(
        ProtocolState::genesis(),
        Box::new(ZkVmBackend),
    );

    let app_state = AppState {
        engine: Arc::new(Mutex::new(engine)),
    };

    let prove_limiter = Arc::new(RateLimiter::new(5, Duration::from_secs(60)));
    let verify_limiter = Arc::new(RateLimiter::new(30, Duration::from_secs(60)));

    let demo_routes = Router::new()
        .route(
            "/demo/prove",
            post(demo_prove_handler)
                .route_layer(middleware::from_fn(RateLimiter::middleware))
                .route_layer(Extension(prove_limiter)),
        )
        .route(
            "/demo/verify",
            post(demo_verify_handler)
                .route_layer(middleware::from_fn(RateLimiter::middleware))
                .route_layer(Extension(verify_limiter)),
        );

    let mut app = Router::new()
        .merge(demo_routes)
        .layer(Extension(app_state));

    if env::var("ZKCG_ENABLE_PROTOCOL").is_ok() {
        println!("[CONFIG] protocol endpoints ENABLED");
        app = app
            .route("/v1/submit-proof", post(submit_proof))
            .route("/v1/prove", post(prove));
    } else {
        println!("[CONFIG] protocol endpoints DISABLED");
    }

    // 👇 global request logging
    let app = app.layer(middleware::from_fn(log_requests));

    println!("[LISTENING] {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
