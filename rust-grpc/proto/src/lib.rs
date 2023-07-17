pub mod svc {
    tonic::include_proto!("hello");
}

pub mod messages {
    pub mod hello {
        pub mod v1 {
            tonic::include_proto!("messages.hello.v1");
        }
    }
}
