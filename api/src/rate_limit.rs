use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
    Extension,
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct RateLimiter {
    max: usize,
    window: Duration,
    hits: Arc<Mutex<Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new(max: usize, window: Duration) -> Self {
        Self {
            max,
            window,
            hits: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn middleware(
        Extension(limiter): Extension<Arc<RateLimiter>>,
        req: Request<Body>,
        next: Next,
    ) -> Response {
        {
            let mut hits = limiter.hits.lock().unwrap();
            let now = Instant::now();

            hits.retain(|t| now.duration_since(*t) < limiter.window);

            if hits.len() >= limiter.max {
                return Response::builder()
                    .status(429)
                    .body("rate limit exceeded".into())
                    .unwrap();
            }

            hits.push(now);
        }

        next.run(req).await
    }
}
