use deadpool_redis::{Config, Pool, Runtime};
use diesel::query_builder;
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use std::future::Future;

pub struct Cache {
    pool: Pool,
}

impl Clone for Cache {
    fn clone(&self) -> Self {
        Cache {
            pool: self.pool.clone(),
        }
    }
}

impl Cache {
    pub fn new() -> Result<Cache, String> {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let cfg = Config::from_url(redis_url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

        Ok(Cache { pool })
    }

    pub async fn get<T>(&self, key: &str) -> Option<T>
    where
        T: FromRedisValue,
    {
        let mut conn = self.pool.get().await.unwrap();
        conn.get(key).await.unwrap()
    }

    pub async fn get_expiretime(&self, key: &str) -> usize {
        let mut conn = self.pool.get().await.unwrap();
        let expiretime: usize = redis::cmd("EXPIRETIME")
            .arg(key)
            .query_async(&mut conn)
            .await
            .unwrap();

        expiretime
    }

    pub async fn set<T>(&self, key: &str, value: T)
    where
        T: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.pool.get().await.unwrap();
        let _: () = conn.set(key, value).await.unwrap();
    }

    pub async fn set_keepttl<T>(&self, key: &str, value: T)
    where
        T: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.pool.get().await.unwrap();
        let _: () = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("KEEPTTL")
            .query_async(&mut conn)
            .await
            .unwrap();
    }

    pub async fn set_and_expire<T>(&self, key: &str, value: T, seconds: usize)
    where
        T: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.pool.get().await.unwrap();
        let _: () = conn.set(key, value).await.unwrap();
        conn.expire(key, seconds).await.unwrap()
    }

    pub async fn get_or_set<T, F, Fut>(&self, key: &str, f: F) -> T
    where
        T: FromRedisValue + ToRedisArgs + Send + Sync,
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        if let Some(value) = self.get(key).await {
            value
        } else {
            let value = f().await;
            self.set(key, &value).await;
            value
        }
    }
}
