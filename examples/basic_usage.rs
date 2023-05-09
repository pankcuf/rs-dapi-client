extern crate rs_dapi_client;

use std::error::Error;
use rs_dapi_client::DAPIClient;
use rs_dapi_client::GetStatusRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let client = DAPIClient::new("http://34.213.25.113:3010");

    let mut service = client.core_service();
    let response = service.get_status(GetStatusRequest {})
        .await
        .expect("Expect response");

    println!("RESPONSE={:?}", response);
    Ok(())
}

