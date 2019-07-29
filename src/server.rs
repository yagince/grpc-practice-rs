use super::helloworld;
use super::helloworld_grpc;
use futures::Future;
use grpcio::{RpcContext, UnarySink};
use log::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Server;

impl helloworld_grpc::Helloworld for Server {
    fn call(
        &mut self,
        ctx: RpcContext,
        req: helloworld::Request,
        sink: UnarySink<helloworld::Response>,
    ) {
        let mut res = helloworld::Response::default();
        res.set_message(format!(
            "Hello name: {} age: {}",
            req.get_name(),
            req.get_age()
        ));

        ctx.spawn(
            sink.success(res)
                .map_err(move |e| error!("failed to reply {:#?}: {:?}", req, e)),
        );
    }
}
