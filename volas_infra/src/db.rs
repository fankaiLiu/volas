use configs::{loader_config::Configurable, Configs};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio::sync::OnceCell;

use crate::error;

pub trait DbService<T: 'static> {
    fn init() -> impl std::future::Future<Output = error::Result<()>> + Send;
    fn pool() -> impl std::future::Future<Output = error::Result<&'static T>> + Send;
}

pub struct SurrealdbServiceImpl;

impl DbService<Surreal<surrealdb::engine::remote::ws::Client>> for SurrealdbServiceImpl {
    async fn init() -> error::Result<()> {
        let config = Configs::config();
        DATABASE_SERVICE
            .get_or_init(|| async {
                let db = Surreal::new::<Ws>("127.0.0.1:8000")
                    .await
                    .expect("db is not available");
                db.signin(Root {
                    username: &config.surrealdb.username,
                    password: &config.surrealdb.password,
                })
                .await
                .expect("db is not available");
                db.use_ns(&config.surrealdb.ns)
                    .use_db(&config.surrealdb.db)
                    .await
                    .expect("use_ns or use db erroe");
                db
            })
            .await;
        Ok(())
    }

    async fn pool() -> error::Result<&'static Surreal<surrealdb::engine::remote::ws::Client>> {
        let pool = DATABASE_SERVICE.get().unwrap();
        Ok(pool)
    }
}

pub static DATABASE_SERVICE: OnceCell<Surreal<surrealdb::engine::remote::ws::Client>> =
    OnceCell::const_new();
