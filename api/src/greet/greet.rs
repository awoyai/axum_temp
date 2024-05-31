use axum::extract::Query;
use repo::common::res::CODE_FAIL_INTERNAL;
use repo::greet::model::greeter::{GreeterInfo, GreeterReq};
use repo::{
    common::res::Res,
    db::{db_conn, DB},
};
use service::greet::greeter;

pub async fn get_by_id(Query(req): Query<GreeterReq>) -> Res<GreeterInfo> {
    let db = DB.get_or_init(db_conn).await;
    match req.id {
        Some(id) => match greeter::get_by_id(db, id).await {
            Err(e) => Res::with_err(CODE_FAIL_INTERNAL, &e.to_string()),
            Ok(res) => Res::with_data(res),
        },
        None => Res::with_msg("用户id不能为空"),
    }
}

pub async fn ping(Query(_): Query<()>) -> Res<()> {
    Res::default()
}
