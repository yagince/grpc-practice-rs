# gRPC in Rust Sample

## Requirements

- Rust > 1.36
- protoc > 3.7.1

## Installation

macOS

```shell
$ brew install protobuf
```

## Build

```shell
$ cargo build
```

## Run

Server

```shell
$ cargo run --bin server
```

Client

```shell
$ cargo run --bin client
```

Client (Ruby)

```shell
$ cd ruby && bundle
$ bundle exec ruby client.rb
```
