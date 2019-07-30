use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::{sync::oneshot, Future};
use grpc_practice::{helloworld_grpc::create_helloworld, Server};
use grpcio::{Environment, ServerBuilder};
use log::*;

fn main() {
    std::env::set_var("RUST_LOG", "server=info,grpc_practice=debug");
    env_logger::init();

    let env = Arc::new(Environment::new(1));
    let service = create_helloworld(Server);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .build()
        .unwrap();

    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
