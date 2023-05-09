use std::time::Duration;
use tonic::transport::Channel;

pub use rs_dapi_client::platform_client::PlatformClient;
pub use rs_dapi_client::core_client::CoreClient;
pub use rs_dapi_client::GetStatusRequest;

/// Contains the generated gRPC stubs for the Dash Platform and Core.
pub mod rs_dapi_client {
    include!("./stubs/core/org.dash.platform.dapi.v0.rs");
    include!("./stubs/platform/org.dash.platform.dapi.v0.rs");
}

/// Alias for the gRPC CoreClient with a tonic::transport::Channel.
type DAPICoreService = CoreClient<Channel>;

/// Alias for the gRPC PlatformClient with a tonic::transport::Channel.
type DAPIPlatformService = PlatformClient<Channel>;

/// DAPIClient is a high-level client for interacting with the Dash Platform and Core.
/// # Examples
///
/// This example shows how to create an instance of DAPIClient and execute get status method of core grpc service.
///
/// ```no_run
/// use rs_dapi_client::DAPIClient;
/// use rs_dapi_client::GetStatusRequest;
///
/// let client = DAPIClient::new("http://34.213.25.113:3010");
///
/// let mut service = client.core_service();
/// let response = service.get_status(GetStatusRequest {})
/// .await
/// .expect("Expect response");
///
/// println!("RESPONSE={:?}", response);
/// ```
///
/// [`DAPIClient`]: struct@self::DAPIClient
/// [`GetStatusRequest`]: struct@crate::rs_dapi_client::GetStatusRequest
pub struct DAPIClient {
    host: String
}

impl DAPIClient {
    /// Creates a new DAPIClient instance with the specified host.
    ///
    /// # Arguments
    ///
    /// * `host` - The host address for the Dash Core and Platform services.
    pub fn new(host: &'static str) -> Self {
        Self {
            host: host.to_string()
        }
    }

    /// Returns a DAPICoreService instance for interacting with the Dash Core.
    pub fn core_service(&self) -> DAPICoreService {
        CoreClient::new(self.grpc_channel())
    }

    /// Returns a DAPIPlatformService instance for interacting with the Dash Platform.
    pub fn platform_service(&self) -> DAPIPlatformService {
        PlatformClient::new(self.grpc_channel())
    }

    /// Creates a tonic::transport::Channel for gRPC communication with the specified host.
    ///
    /// # Returns
    ///
    /// A tonic::transport::Channel instance for gRPC communication.
    fn grpc_channel(&self) -> Channel {
        let host = self.host.to_string();
        let channel = tonic::transport::Endpoint::from_shared(host)
            .expect("Expect endpoint")
            .keep_alive_timeout(Duration::from_secs(30))
            .http2_keep_alive_interval(Duration::from_secs(60))
            .keep_alive_while_idle(true)
            .connect_lazy();
        channel
    }
}
