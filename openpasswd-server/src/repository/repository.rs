use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::{info, warn};

diesel_migrations::embed_migrations!("../migrations");

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

    pub fn migration_run(&self) {
        info!("Applying migrations");
        let conn = self.pool.get().unwrap();
        embedded_migrations::run(&conn).unwrap();
        // embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();
    }
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let mut builder = Pool::builder().test_on_check_out(true);

    match std::env::var("DATABASE_POOL_SIZE") {
        Ok(max_size) => match max_size.parse::<u32>() {
            Ok(max_size) => {
                builder = builder.max_size(max_size);
            }
            Err(e) => warn!("DATABASE_POOL_SIZE: {e}"),
        },
        Err(e) => info!("DATABASE_POOL_SIZE: {e}; default 10"),
    }

    builder
        .build(manager)
        .expect("Could not build connection pool")
}
