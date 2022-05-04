use deadpool_redis::{Config, Pool, Runtime};
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};

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
        let cfg = Config::from_url("redis://127.0.0.1:6379/");
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

    pub async fn set<T>(&self, key: &str, value: T)
    where
        T: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.pool.get().await.unwrap();
        let _: () = conn.set(key, value).await.unwrap();
    }

    pub async fn set_and_expire<T>(&self, key: &str, value: T, seconds: usize)
    where
        T: ToRedisArgs + Send + Sync,
    {
        let mut conn = self.pool.get().await.unwrap();
        let _: () = conn.set(key, value).await.unwrap();
        conn.expire(key, seconds).await.unwrap()
    }
}
