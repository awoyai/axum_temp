use axum::{routing::post, Router};

pub mod greet;

pub fn greet_api() -> Router {
    Router::new().route("/greet", post(greet::get_by_id))
}
