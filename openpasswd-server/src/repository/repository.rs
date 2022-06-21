use log::{info, warn};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};
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

    pub async fn migration_run(&self) {
        info!("Applying migrations");
        Migrator::up(&self.db, None)
            .await
            .expect("Could not apply migrations");
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
