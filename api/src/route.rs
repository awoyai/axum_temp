use axum::{routing::get, Router};

use crate::greet;

pub fn api() -> Router {
    Router::new()
        // 无需授权Api.通用模块
        .nest("/comm", no_auth_api())
        // 系统管理模块
        .nest("/system", set_auth_middleware(greet::greet_api()))
}

fn no_auth_api() -> Router {
    Router::new().route("/ping", get(greet::ping)) // 登录
}

fn set_auth_middleware(router: Router) -> Router {
    router
}
