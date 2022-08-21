// use tonic::{transport::Server, Request, Response, Status};

// pub mod say {
//     tonic::include_proto!("say");
// }

// #[tokio::main]
// async fn main() {}

pub mod smartcard {
    tonic::include_proto!("smartcard");
}

fn main() {
    let request = smartcard::SmartcardInfoRequest {
        name: "new".to_string(),
        timestamp: 0,
    };

    println!("{:?}", request);

    let test = smartcard::Test {
        test: "test".to_string(),
    };
    // smartcard::smartcard_service_client::SmartcardServiceClien
}
