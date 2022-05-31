use log::{info, warn};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

// TODO:
// diesel_migrations::embed_migrations!("../migrations");

pub struct Repository {
    pub db: DatabaseConnection,
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Repository {
            db: self.db.clone(),
        }
    }
}

impl Repository {
    pub async fn new() -> Repository {
        Repository {
            db: get_connection_pool().await,
        }
    }

    pub fn migration_run(&self) {
        // TODO:
        // info!("Applying migrations");
        // let conn = self.pool.get().unwrap();
        // embedded_migrations::run(&conn).unwrap();
        // // embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();
    }
}

pub async fn get_connection_pool() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let max_connections = match std::env::var("DATABASE_POOL_SIZE") {
        Ok(max_connections) => match max_connections.parse::<u32>() {
            Ok(max_connections) => max_connections,
            Err(e) => {
                warn!("DATABASE_POOL_SIZE: {e}");
                10
            }
        },
        Err(e) => {
            info!("DATABASE_POOL_SIZE: {e}; default 10");
            10
        }
    };

    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(max_connections)
        .min_connections(5)
        // TODO
        // .connect_timeout(Duration::from_secs(8))
        // .idle_timeout(Duration::from_secs(8))
        // .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    Database::connect(opt)
        .await
        .expect("Could not build connection pool")
}
