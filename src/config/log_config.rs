use axum::body::Body;
use axum::http::{Request, Response};
use std::time::Duration;
use tracing::{Span, info};

pub fn log_request<'a, 'b>(request: &'a Request<Body>, _span: &'b Span) {
    info!(
        "Incoming request: {} {}",
        request.method(),
        request.uri().path()
    );
}

pub fn log_response<'a, 'b>(response: &'a Response<Body>, latency: Duration, _span: &'b Span) {
    info!(
        "Response status: {} (processed in {:?})",
        response.status(),
        latency
    );
}
