use std::sync::Arc;

use grpc_practice::helloworld::Request;
use grpc_practice::helloworld_grpc::HelloworldClient;
use grpcio::{ChannelBuilder, EnvBuilder};
use log::*;

fn main() {
    std::env::set_var("RUST_LOG", "client=info");
    env_logger::init();

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");

    let client = HelloworldClient::new(ch);

    let mut req = Request::default();
    req.set_name("world".to_owned());
    req.set_age(100);
    let reply = client.call(&req).expect("rpc");
    info!("Helloworld received: {}", reply.get_message());
}
