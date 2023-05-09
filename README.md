# Rust DAPI Client

`rs-dapi-client` is a Rust library for interacting with the Dash Platform and Core services using gRPC.

## Features

- Provides convenient access to Dash Platform and Core services.
- Includes generated gRPC stubs for Dash Platform and Core.
- Easy-to-use API for creating gRPC clients for Dash Platform and Core.

## Prerequisites

Before using this library, ensure that you have the following:

- [Rust](https://www.rust-lang.org/) installed.
- A Dash Core node with gRPC enabled.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rs-dapi-client = { git = "https://github.com/tikhop/rs-dapi-client.git" }
```

Then, run `cargo build` to fetch and build the library.

## Usage

Here's a simple example of using `rs-dapi-client` to interact with Dash Platform and Core services.

First, include the library:

```rust
use rs_dapi_client::{DAPIClient, DAPICoreService, DAPIPlatformService, GetStatusRequest};
```

Then, create a new `DAPIClient` instance with the host address of your Dash Core node:

```rust
let client = DAPIClient::new("http://localhost:3000");
```

Now, you can create gRPC clients for the Dash Platform and Core services:

```rust
let core_service: DAPICoreService = client.core_service();
let platform_service: DAPIPlatformService = client.platform_service();
```

Here's an example of using the `core_service` to get the status of the Dash Core:

```rust
use tonic::Request;

async fn get_status(core_service: &DAPICoreService) {
    let request = Request::new(GetStatusRequest {});
    match core_service.get_status(request).await {
        Ok(response) => println!("Dash Core status: {:?}", response.into_inner()),
        Err(error) => eprintln!("Error fetching Dash Core status: {:?}", error),
    }
}
```

## Examples

Check the `examples` directory for additional usage examples.

## License

This project is licensed under the [MIT License](LICENSE).