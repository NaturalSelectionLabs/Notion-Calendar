mod calendar;
mod database;
mod notion;
mod server;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = ([0, 0, 0, 0], 3000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(server::echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
