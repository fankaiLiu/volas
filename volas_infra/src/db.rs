use std::{collections::HashMap, time::Duration};
use async_trait::async_trait;
use configs::CFG;
use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use tokio::sync::OnceCell;
pub static DATABASE_SERVICE: OnceCell<DatabaseServiceImpl> = OnceCell::const_new();

pub async fn init_db_conn() {
    DATABASE_SERVICE.get_or_init(|| async {
        let mut db_service = DatabaseServiceImpl::new();
        db_service.init_db_conn().await;
        db_service
    })
    .await;
}

pub struct DatabaseServiceImpl {
    db: HashMap<String, DatabaseConnection>,
}

#[async_trait]
pub trait DatabaseService {
    fn new() -> Self;
    async fn init_db_conn(&mut self);
    fn get_db_conn<T: Into<String>>(&self, name: T) -> Option<&DatabaseConnection>;
}
#[async_trait]
impl DatabaseService for DatabaseServiceImpl {
    fn new() -> Self {
        Self { db: HashMap::new() }
    }

    async fn init_db_conn(&mut self) {
        for (name, url) in CFG.database.database_urls.iter() {
            let mut opt = ConnectOptions::new(url.to_owned());
            opt.max_connections(1000)
                .min_connections(5)
                .connect_timeout(Duration::from_secs(8))
                .idle_timeout(Duration::from_secs(8))
                .sqlx_logging(false);

            let db = Database::connect(opt)
                .await
                .expect("Database connection failure");
            self.db.insert(name.clone(), db);
        }
    }

    fn get_db_conn<T: Into<String>>(&self, name: T) -> Option<&DatabaseConnection> {
        self.db.get(&name.into())
    }
}
