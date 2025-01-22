use tonic::Request;
use dispatcher::protobuf::{greeter_client, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server at localhost:50051
    let mut client = greeter_client::GreeterClient::connect("http://[::1]:50051").await?;

    // Create a request
    let request = Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    // Call the gRPC method
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}