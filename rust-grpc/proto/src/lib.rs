#[path = "api/hello.rs"]
pub mod svc;

#[path = "api/"]
pub mod messages {
    #[path = ""]
    pub mod hello {
        #[path = "messages.hello.v1.rs"]
        pub mod v1;
        
        #[path = "messages.hello.v1.rs"]
        pub mod v2;
    }
}
