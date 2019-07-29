// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_HELLOWORLD_CALL: ::grpcio::Method<super::helloworld::Request, super::helloworld::Response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/example.helloworld.Helloworld/Call",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct HelloworldClient {
    client: ::grpcio::Client,
}

impl HelloworldClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        HelloworldClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn call_opt(&self, req: &super::helloworld::Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::helloworld::Response> {
        self.client.unary_call(&METHOD_HELLOWORLD_CALL, req, opt)
    }

    pub fn call(&self, req: &super::helloworld::Request) -> ::grpcio::Result<super::helloworld::Response> {
        self.call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn call_async_opt(&self, req: &super::helloworld::Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::helloworld::Response>> {
        self.client.unary_call_async(&METHOD_HELLOWORLD_CALL, req, opt)
    }

    pub fn call_async(&self, req: &super::helloworld::Request) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::helloworld::Response>> {
        self.call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Helloworld {
    fn call(&mut self, ctx: ::grpcio::RpcContext, req: super::helloworld::Request, sink: ::grpcio::UnarySink<super::helloworld::Response>);
}

pub fn create_helloworld<S: Helloworld + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_HELLOWORLD_CALL, move |ctx, req, resp| {
        instance.call(ctx, req, resp)
    });
    builder.build()
}
