# rust-grpc

A simple gRPC client and server written in Rust, using the [tonic](https://docs.rs/tonic/latest/tonic/) crate.

### Directory structure

```bash
rust-grpc on main [!?] via 🐹 v1.20.2 via 🦀 v1.69.0 at ☸️  kind-kind
➜ tree . -d
.
├── grpc-client     # gRPC client (bin)
│   └── src
├── grpc-server     # gRPC server (bin)
│   └── src
└── proto           # gRPC proto files (lib)
    └── src
        └── protos
            ├── hello
            └── messages
```

### Dependencies

- tonic (v0.9)
- prost (v0.11)
- tokio (v1.0)
- tonic-build (v0.9)

### Run Client and Server

In one terminal window, run the server:

```bash
cargo run -p grpc-server
```

In another terminal window, run the client:

```bash
cargo run -p grpc-client
```

### Commit messages

The commit messages have been generated using [whatthecommit](https://whatthecommit.com/).
