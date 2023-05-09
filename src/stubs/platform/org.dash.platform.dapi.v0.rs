#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proof {
    #[prost(bytes = "vec", tag = "1")]
    pub merkle_proof: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub quorum_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "4")]
    pub round: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadata {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(uint32, tag = "2")]
    pub core_chain_locked_height: u32,
    #[prost(uint64, tag = "3")]
    pub time_ms: u64,
    #[prost(uint32, tag = "4")]
    pub protocol_version: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateTransitionBroadcastError {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastStateTransitionRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub state_transition: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastStateTransitionResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdentityRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub prove: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdentityResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub identity: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub proof: ::core::option::Option<Proof>,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataContractRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub prove: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataContractResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data_contract: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub proof: ::core::option::Option<Proof>,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentsRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data_contract_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub document_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub r#where: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub order_by: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "5")]
    pub limit: u32,
    #[prost(bool, tag = "8")]
    pub prove: bool,
    #[prost(oneof = "get_documents_request::Start", tags = "6, 7")]
    pub start: ::core::option::Option<get_documents_request::Start>,
}
/// Nested message and enum types in `GetDocumentsRequest`.
pub mod get_documents_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Start {
        #[prost(bytes, tag = "6")]
        StartAfter(::prost::alloc::vec::Vec<u8>),
        #[prost(bytes, tag = "7")]
        StartAt(::prost::alloc::vec::Vec<u8>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentsResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "2")]
    pub proof: ::core::option::Option<Proof>,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdentitiesByPublicKeyHashesRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub public_key_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, tag = "2")]
    pub prove: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdentitiesByPublicKeyHashesResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub identities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "2")]
    pub proof: ::core::option::Option<Proof>,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForStateTransitionResultRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub state_transition_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub prove: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForStateTransitionResultResponse {
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
    #[prost(
        oneof = "wait_for_state_transition_result_response::Responses",
        tags = "1, 2"
    )]
    pub responses: ::core::option::Option<
        wait_for_state_transition_result_response::Responses,
    >,
}
/// Nested message and enum types in `WaitForStateTransitionResultResponse`.
pub mod wait_for_state_transition_result_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Responses {
        #[prost(message, tag = "1")]
        Error(super::StateTransitionBroadcastError),
        #[prost(message, tag = "2")]
        Proof(super::Proof),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusParamsBlock {
    #[prost(string, tag = "1")]
    pub max_bytes: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub max_gas: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub time_iota_ms: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusParamsEvidence {
    #[prost(string, tag = "1")]
    pub max_age_num_blocks: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub max_age_duration: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub max_bytes: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsensusParamsRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(bool, tag = "2")]
    pub prove: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsensusParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<ConsensusParamsBlock>,
    #[prost(message, optional, tag = "2")]
    pub evidence: ::core::option::Option<ConsensusParamsEvidence>,
}
/// Generated client implementations.
pub mod platform_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlatformClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlatformClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlatformClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PlatformClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlatformClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn broadcast_state_transition(
            &mut self,
            request: impl tonic::IntoRequest<super::BroadcastStateTransitionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BroadcastStateTransitionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.dash.platform.dapi.v0.Platform/broadcastStateTransition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "org.dash.platform.dapi.v0.Platform",
                        "broadcastStateTransition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_identity(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIdentityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetIdentityResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.dash.platform.dapi.v0.Platform/getIdentity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("org.dash.platform.dapi.v0.Platform", "getIdentity"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_data_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataContractRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDataContractResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.dash.platform.dapi.v0.Platform/getDataContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "org.dash.platform.dapi.v0.Platform",
                        "getDataContract",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDocumentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.dash.platform.dapi.v0.Platform/getDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("org.dash.platform.dapi.v0.Platform", "getDocuments"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_identities_by_public_key_hashes(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetIdentitiesByPublicKeyHashesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetIdentitiesByPublicKeyHashesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.dash.platform.dapi.v0.Platform/getIdentitiesByPublicKeyHashes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "org.dash.platform.dapi.v0.Platform",
                        "getIdentitiesByPublicKeyHashes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn wait_for_state_transition_result(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitForStateTransitionResultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WaitForStateTransitionResultResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.dash.platform.dapi.v0.Platform/waitForStateTransitionResult",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "org.dash.platform.dapi.v0.Platform",
                        "waitForStateTransitionResult",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_consensus_params(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConsensusParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetConsensusParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.dash.platform.dapi.v0.Platform/getConsensusParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "org.dash.platform.dapi.v0.Platform",
                        "getConsensusParams",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
