use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

//Nuke happened here but we got POOLS :DDDDDD
#[async_trait]
pub trait SqlStruct<T> {
    async fn add(&self, pool: &PgPool) -> Result<T>;
}
