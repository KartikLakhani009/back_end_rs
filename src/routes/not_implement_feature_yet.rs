use axum::{
    body::Body,
    http::Request,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
#[allow(unused)]
#[allow(clippy::unused_async)]
pub async fn test_route() -> Router {
    Router::new().route("/test", get(not_implemented_route))
}

pub async fn not_implemented_route(req: Request<Body>) -> impl IntoResponse {
    // add which route is requesting this?
    Html(format!(
        "Route is planned but not yet implemented for {}",
        req.uri()
    ))
}
