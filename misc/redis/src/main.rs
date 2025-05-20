use std::env;
use deadpool_redis::{redis::{cmd, FromRedisValue}, sentinel::SentinelServerType};
use deadpool_redis::sentinel::{Config, Runtime};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    use deadpool_redis::redis::pipe;
    let redis_urls = env::var("REDIS_SENTINEL__URLS")
        .unwrap()
        .split(',')
        .map(String::from)
        .collect::<Vec<_>>();

    let master_name = env::var("REDIS_SENTINEL__MASTER_NAME").unwrap();
    let mut cfg = Config::from_urls(redis_urls, master_name, SentinelServerType::Master);
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

    let mut conn = pool.get().await.unwrap();
    let (value,): (String,) = pipe()
        .cmd("SET")
        .arg("deadpool/pipeline_test_key")
        .arg("42")
        .ignore()
        .cmd("GET")
        .arg("deadpool/pipeline_test_key")
        .query_async(&mut conn)
        .await
        .unwrap();
    assert_eq!(value, "42".to_string());
}