use configs::loader_config::ConfigBuilder;
use infra::init_db_conn;


#[tokio::main]
async fn main() {
    ConfigBuilder::new().build();
    init_db_conn().await;
    println!("Hello, world!");
}
