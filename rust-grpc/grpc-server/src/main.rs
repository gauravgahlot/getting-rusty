use proto::messages::hello::{v1, v2};
use proto::svc::greeter_server::{Greeter, GreeterServer};

use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<v1::Request>, // Accept request of type Request.
    ) -> Result<Response<v1::Response>, Status> {
        // Return an instance of type Response.
        println!("Got a request: {:?}", request);

        let reply = v1::Response {
            // We must use .into_inner() as the fields of gRPC requests and responses are private.
            message: format!("Hello, {}!", request.into_inner().name).into(),
        };

        // Send back formatted greeting.
        Ok(Response::new(reply))
    }

    async fn wave_off(
        &self,
        request: Request<v2::Request>,
    ) -> Result<Response<v2::Response>, Status> {
        println!("Got a request: {:?}", request);

        let reply = v2::Response {
            message: format!("See you soon, {}!", request.into_inner().name).into(),
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
