use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

use crate::greet;

pub fn api() -> Router {
    Router::new()
        // 无需授权Api.通用模块
        .nest("/comm", set_middleware(no_auth_api()))
        // 系统管理模块
        .nest("/system", set_auth_middleware(greet::greet_api()))
}

fn no_auth_api() -> Router {
    Router::new().route("/ping", get(greet::greet::ping)) // 登录
}

fn set_auth_middleware(router: Router) -> Router {
    router
}

fn set_middleware(router: Router) -> Router {
    let router = router.layer(middleware::from_fn(m));
    router
}

pub async fn m(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    req.headers_mut().insert("b", "world".parse().unwrap());
    Ok(next.run(req).await)
}
