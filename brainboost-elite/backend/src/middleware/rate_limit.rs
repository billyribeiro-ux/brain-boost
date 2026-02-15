use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::limit::RequestBodyLimitLayer;

pub fn rate_limit_layer() -> ServiceBuilder<tower::layer::util::Stack<RequestBodyLimitLayer, tower::layer::util::Identity>> {
    ServiceBuilder::new()
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
}
