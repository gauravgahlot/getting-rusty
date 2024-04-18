use proto::messages::hello::{v1, v2};
use proto::svc::greeter_client::GreeterClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(v1::Request {
        name: "Ferris".into(),
    });

    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);

    let request = tonic::Request::new(v2::Request {
        name: "Ferris".into(),
    });

    let response = client.wave_off(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
