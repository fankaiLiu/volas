use configs::CFG;
use infra::init_db_conn;
use salvo::{conn::TcpListener, Listener, Router, Server, Service};
use system::System;
use tokio::sync::oneshot;
use web::Routers;

#[tokio::main]
async fn main() {
    dbg!(&CFG.server.name);
    init_db_conn().await;
    let service: Service = Router::new().append(&mut System.build()).into();
    println!("Starting server at {}", &CFG.server.address);
    let (tx, rx) = oneshot::channel();
    let acceptor = TcpListener::new(&CFG.server.address).bind().await;
    let server = Server::new(acceptor).serve_with_graceful_shutdown(
        service,
        async {
            rx.await.ok();
        },
        None,
    );
    tokio::task::spawn(server);
    tokio::signal::ctrl_c().await.unwrap();
    let _ = tx.send(());
}
