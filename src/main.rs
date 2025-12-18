use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello");
}

use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};

#[derive(Default)]
struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;

        let reply = HelloReply {
            message: format!("Hello {}!", name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("gRPC server running on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
