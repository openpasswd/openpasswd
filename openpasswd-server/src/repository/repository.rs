use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::{info, warn};

pub struct Repository {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Repository {
            pool: self.pool.clone(),
        }
    }
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            pool: get_connection_pool(),
        }
    }
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .max_size(
            std::env::var("DATABASE_POOL_SIZE")
                .unwrap_or_else(|e| {
                    info!("DATABASE_POOL_SIZE: {e}; default 5");
                    "5".to_owned()
                })
                .parse::<u32>()
                .unwrap_or_else(|e| {
                    warn!("DATABASE_POOL_SIZE: {e}");
                    5
                }),
        )
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
