mod calendar;
mod database;
mod notion;
mod server;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(server::echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
