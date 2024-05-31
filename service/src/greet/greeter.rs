use anyhow::{anyhow, Result};
use repo::greet::entities::prelude::Greeter;
use repo::greet::model::greeter::GreeterInfo;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_by_id(db: &DatabaseConnection, id: i64) -> Result<GreeterInfo> {
    let greeter = match Greeter::find_by_id(id).one(db).await? {
        None => return Err(anyhow!("数据不存在")),
        Some(info) => GreeterInfo {
            id: info.id,
            name: info.name,
        },
    };
    Ok(greeter)
}
