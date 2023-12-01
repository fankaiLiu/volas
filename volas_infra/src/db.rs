use std::{time::Duration, collections::HashMap};

use configs::CFG;
use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use tokio::sync::OnceCell;
pub static DB: OnceCell<HashMap<String,DatabaseConnection>> = OnceCell::const_new();

pub(crate) async fn init_db_conn() {
	DB.get_or_init(|| async {

		let mut dbs = HashMap::new();
        for (name, url) in CFG.database.database_urls.iter() {
            let mut opt = ConnectOptions::new(url.to_owned());
            opt.max_connections(1000)
                .min_connections(5)
                .connect_timeout(Duration::from_secs(8))
                .idle_timeout(Duration::from_secs(8))
                .sqlx_logging(false);

            let db = Database::connect(opt).await.expect("Database connection failure");
            dbs.insert(name.clone(), db);
        }
        dbs
	})
	.await;
}
