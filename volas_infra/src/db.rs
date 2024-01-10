use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio::sync::OnceCell;

pub static DATABASE_SERVICE: OnceCell<Surreal<surrealdb::engine::remote::ws::Client>> =
    OnceCell::const_new();

pub async fn init_db_conn() {
    DATABASE_SERVICE
        .get_or_init(|| async {
            let db = Surreal::new::<Ws>("127.0.0.1:8000")
                .await
                .expect("db is not available");
            db.signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .expect("db is not available");
            db.use_ns("test")
                .use_db("test")
                .await
                .expect("use_ns or use db erroe");
            db
        })
        .await;
}
