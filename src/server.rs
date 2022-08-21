use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod smartcard {
    tonic::include_proto!("smartcard");
}

use smartcard::smartcard_service_server::SmartcardService;

#[derive(Debug, Default)]
pub struct MySmartcard {}

#[tonic::async_trait]
impl SmartcardService for MySmartcard {
    async fn send_smartcard_info(
        &self,
        request: Request<smartcard::SmartcardInfoRequest>,
    ) -> Result<Response<smartcard::SmartcardInfoResponse>, Status> {
        println!("Got a request");

        let reply = smartcard::SmartcardInfoResponse { ok: true };

        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
