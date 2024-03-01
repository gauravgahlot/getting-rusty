use proto::messages::hello::v1::{HelloRequest, HelloResponse};
use proto::svc::greeter_server::{Greeter, GreeterServer};

use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest.
    ) -> Result<Response<HelloResponse>, Status> { // Return an instance of type HelloResponse.
        println!("Got a request: {:?}", request);

        let reply = HelloResponse {
            // We must use .into_inner() as the fields of gRPC requests and responses are private.
            message: format!("Hello, {}!", request.into_inner().name).into(),
        };

        // Send back formatted greeting.
        Ok(Response::new(reply)) 
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("gRPC-server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;  

    Ok(())
}

