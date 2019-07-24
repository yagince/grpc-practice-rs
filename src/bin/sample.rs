fn main() {
    let mut req = grpc_practice_rs::helloworld::Request::new();
    req.set_name("test".into());
    req.set_age(100);
    dbg!(req);
}
