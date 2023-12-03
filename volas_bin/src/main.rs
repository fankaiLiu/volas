use configs::CFG;
use infra::init_db_conn;

#[tokio::main]
async fn main() {
    dbg!(&CFG.server.name);
    init_db_conn().await;
    println!("Hello, world!");
}
