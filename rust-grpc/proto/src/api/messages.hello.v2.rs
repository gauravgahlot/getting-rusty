/// The request message containing the user's name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response message containing the greetings
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
