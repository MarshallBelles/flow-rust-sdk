// ****************************************************
// Welcome to the flow-rust-sdk!
// Please read the license file
// ****************************************************

// ****************************************************
// External Dependencies
// ****************************************************
use std::error;

use flow::api_client::ApiClient;

use flow::{
    AccountResponse, BlockResponse, CollectionResponse, EventsResponse,
    ExecuteScriptAtLatestBlockRequest, ExecuteScriptResponse, GetAccountAtLatestBlockRequest,
    GetBlockByHeightRequest, GetBlockByIdRequest, GetCollectionByIdRequest,
    GetEventsForBlockIdsRequest, GetEventsForHeightRangeRequest, GetLatestBlockRequest,
    PingRequest, SendTransactionRequest, SendTransactionResponse, Transaction,
    TransactionProposalKey, TransactionSignature,
};

pub mod flow {

    /// ping
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PingRequest {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PingResponse {}
    /// block headers
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockHeaderResponse {
        #[prost(message, optional, tag = "1")]
        pub block: ::core::option::Option<BlockHeader>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetLatestBlockHeaderRequest {
        #[prost(bool, tag = "1")]
        pub is_sealed: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetBlockHeaderByIdRequest {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetBlockHeaderByHeightRequest {
        #[prost(uint64, tag = "1")]
        pub height: u64,
    }
    /// blocks
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockResponse {
        #[prost(message, optional, tag = "1")]
        pub block: ::core::option::Option<Block>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetLatestBlockRequest {
        #[prost(bool, tag = "1")]
        pub is_sealed: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetBlockByIdRequest {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetBlockByHeightRequest {
        #[prost(uint64, tag = "1")]
        pub height: u64,
    }
    /// collections
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CollectionResponse {
        #[prost(message, optional, tag = "1")]
        pub collection: ::core::option::Option<Collection>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetCollectionByIdRequest {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
    }
    /// transactions
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SendTransactionResponse {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SendTransactionRequest {
        #[prost(message, optional, tag = "1")]
        pub transaction: ::core::option::Option<Transaction>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetTransactionRequest {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionResponse {
        #[prost(message, optional, tag = "1")]
        pub transaction: ::core::option::Option<Transaction>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionResultResponse {
        #[prost(enumeration = "TransactionStatus", tag = "1")]
        pub status: i32,
        #[prost(uint32, tag = "2")]
        pub status_code: u32,
        #[prost(string, tag = "3")]
        pub error_message: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "4")]
        pub events: ::prost::alloc::vec::Vec<Event>,
    }
    /// accounts
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccountResponse {
        #[prost(message, optional, tag = "1")]
        pub account: ::core::option::Option<Account>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetAccountAtLatestBlockRequest {
        #[prost(bytes = "vec", tag = "1")]
        pub address: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetAccountAtBlockHeightRequest {
        #[prost(bytes = "vec", tag = "1")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint64, tag = "2")]
        pub block_height: u64,
    }
    /// scripts
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecuteScriptResponse {
        #[prost(bytes = "vec", tag = "1")]
        pub value: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecuteScriptAtLatestBlockRequest {
        #[prost(bytes = "vec", tag = "1")]
        pub script: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecuteScriptAtBlockIdRequest {
        #[prost(bytes = "vec", tag = "1")]
        pub block_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "2")]
        pub script: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecuteScriptAtBlockHeightRequest {
        #[prost(uint64, tag = "1")]
        pub block_height: u64,
        #[prost(bytes = "vec", tag = "2")]
        pub script: ::prost::alloc::vec::Vec<u8>,
    }
    /// events
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EventsResponse {
        #[prost(message, repeated, tag = "1")]
        pub results: ::prost::alloc::vec::Vec<events_response::Result>,
    }
    /// Nested message and enum types in `EventsResponse`.
    pub mod events_response {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Result {
            #[prost(bytes = "vec", tag = "1")]
            pub block_id: ::prost::alloc::vec::Vec<u8>,
            #[prost(uint64, tag = "2")]
            pub block_height: u64,
            #[prost(message, repeated, tag = "3")]
            pub events: ::prost::alloc::vec::Vec<super::Event>,
            #[prost(message, optional, tag = "4")]
            pub block_timestamp: ::core::option::Option<super::Timestamp>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetEventsForHeightRangeRequest {
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub start_height: u64,
        #[prost(uint64, tag = "3")]
        pub end_height: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetEventsForBlockIdsRequest {
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    /// network parameters
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetNetworkParametersResponse {
        #[prost(string, tag = "1")]
        pub chain_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetNetworkParametersRequest {}
    /// protocol state
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtocolStateSnapshotResponse {
        #[prost(bytes = "vec", tag = "1")]
        pub serialized_snapshot: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetLatestProtocolStateSnapshotRequest {}
    /// execution results
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutionResultForBlockIdResponse {
        #[prost(message, optional, tag = "1")]
        pub execution_result: ::core::option::Option<ExecutionResult>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetExecutionResultForBlockIdRequest {
        #[prost(bytes = "vec", tag = "1")]
        pub block_id: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Block {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "2")]
        pub parent_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint64, tag = "3")]
        pub height: u64,
        #[prost(message, optional, tag = "4")]
        pub timestamp: ::core::option::Option<Timestamp>,
        #[prost(message, repeated, tag = "5")]
        pub collection_guarantees: ::prost::alloc::vec::Vec<CollectionGuarantee>,
        #[prost(message, repeated, tag = "6")]
        pub block_seals: ::prost::alloc::vec::Vec<BlockSeal>,
        #[prost(bytes = "vec", repeated, tag = "7")]
        pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockHeader {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "2")]
        pub parent_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint64, tag = "3")]
        pub height: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockSeal {
        #[prost(bytes = "vec", tag = "1")]
        pub block_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "2")]
        pub execution_receipt_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", repeated, tag = "3")]
        pub execution_receipt_signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", repeated, tag = "4")]
        pub result_approval_signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Collection {
        #[prost(bytes = "vec", tag = "1")]
        pub id: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub transaction_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CollectionGuarantee {
        #[prost(bytes = "vec", tag = "1")]
        pub collection_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transaction {
        #[prost(bytes = "vec", tag = "1")]
        pub script: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub arguments: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", tag = "3")]
        pub reference_block_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint64, tag = "4")]
        pub gas_limit: u64,
        #[prost(message, optional, tag = "5")]
        pub proposal_key: ::core::option::Option<TransactionProposalKey>,
        #[prost(bytes = "vec", tag = "6")]
        pub payer: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", repeated, tag = "7")]
        pub authorizers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        #[prost(message, repeated, tag = "8")]
        pub payload_signatures: ::prost::alloc::vec::Vec<TransactionSignature>,
        #[prost(message, repeated, tag = "9")]
        pub envelope_signatures: ::prost::alloc::vec::Vec<TransactionSignature>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionProposalKey {
        #[prost(bytes = "vec", tag = "1")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint32, tag = "2")]
        pub key_id: u32,
        #[prost(uint64, tag = "3")]
        pub sequence_number: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionSignature {
        #[prost(bytes = "vec", tag = "1")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint32, tag = "2")]
        pub key_id: u32,
        #[prost(bytes = "vec", tag = "3")]
        pub signature: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Account {
        #[prost(bytes = "vec", tag = "1")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint64, tag = "2")]
        pub balance: u64,
        #[prost(bytes = "vec", tag = "3")]
        pub code: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, repeated, tag = "4")]
        pub keys: ::prost::alloc::vec::Vec<AccountKey>,
        #[prost(map = "string, bytes", tag = "5")]
        pub contracts: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::vec::Vec<u8>,
        >,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccountKey {
        #[prost(uint32, tag = "1")]
        pub id: u32,
        #[prost(bytes = "vec", tag = "2")]
        pub public_key: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint32, tag = "3")]
        pub sign_algo: u32,
        #[prost(uint32, tag = "4")]
        pub hash_algo: u32,
        #[prost(uint32, tag = "5")]
        pub weight: u32,
        #[prost(uint32, tag = "6")]
        pub sequence_number: u32,
        #[prost(bool, tag = "7")]
        pub revoked: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Event {
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(bytes = "vec", tag = "2")]
        pub transaction_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint32, tag = "3")]
        pub transaction_index: u32,
        #[prost(uint32, tag = "4")]
        pub event_index: u32,
        #[prost(bytes = "vec", tag = "5")]
        pub payload: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExecutionResult {
        #[prost(bytes = "vec", tag = "1")]
        pub previous_result_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "2")]
        pub block_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, repeated, tag = "3")]
        pub chunks: ::prost::alloc::vec::Vec<Chunk>,
        #[prost(message, repeated, tag = "4")]
        pub service_events: ::prost::alloc::vec::Vec<ServiceEvent>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Chunk {
        #[prost(bytes = "vec", tag = "1")]
        pub start_state: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "2")]
        pub event_collection: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "3")]
        pub block_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint64, tag = "4")]
        pub total_computation_used: u64,
        #[prost(uint64, tag = "5")]
        pub number_of_transactions: u64,
        #[prost(uint64, tag = "6")]
        pub index: u64,
        #[prost(bytes = "vec", tag = "7")]
        pub end_state: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServiceEvent {
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(bytes = "vec", tag = "2")]
        pub payload: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Timestamp {
        /// Represents seconds of UTC time since Unix epoch
        /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
        /// 9999-12-31T23:59:59Z inclusive.
        #[prost(int64, tag = "1")]
        pub seconds: i64,
        /// Non-negative fractions of a second at nanosecond resolution. Negative
        /// second values with fractions must still have non-negative nanos values
        /// that count forward in time. Must be from 0 to 999,999,999
        /// inclusive.
        #[prost(int32, tag = "2")]
        pub nanos: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransactionStatus {
        Unknown = 0,
        Pending = 1,
        Finalized = 2,
        Executed = 3,
        Sealed = 4,
        Expired = 5,
    }
    #[doc = r" Generated client implementations."]
    pub mod api_client {
        #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
        use tonic::codegen::*;
        #[derive(Debug, Clone)]
        pub struct ApiClient<T> {
            inner: tonic::client::Grpc<T>,
        }
        impl ApiClient<tonic::transport::Channel> {
            #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: std::convert::TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }
        impl<T> ApiClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::ResponseBody: Body + Send + Sync + 'static,
            T::Error: Into<StdError>,
            <T::ResponseBody as Body>::Error: Into<StdError> + Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> ApiClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                    Into<StdError> + Send + Sync,
            {
                ApiClient::new(InterceptedService::new(inner, interceptor))
            }
            #[doc = r" Compress requests with `gzip`."]
            #[doc = r""]
            #[doc = r" This requires the server to support it otherwise it might respond with an"]
            #[doc = r" error."]
            pub fn send_gzip(mut self) -> Self {
                self.inner = self.inner.send_gzip();
                self
            }
            #[doc = r" Enable decompressing responses with `gzip`."]
            pub fn accept_gzip(mut self) -> Self {
                self.inner = self.inner.accept_gzip();
                self
            }
            #[doc = " the following is copied from docs.onflow.org/access-api/"]
            pub async fn ping(
                &mut self,
                request: impl tonic::IntoRequest<super::PingRequest>,
            ) -> Result<tonic::Response<super::PingResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/flowrust.Api/Ping");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_latest_block_header(
                &mut self,
                request: impl tonic::IntoRequest<super::GetLatestBlockHeaderRequest>,
            ) -> Result<tonic::Response<super::BlockHeaderResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path =
                    http::uri::PathAndQuery::from_static("/flowrust.Api/GetLatestBlockHeader");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_block_header_by_id(
                &mut self,
                request: impl tonic::IntoRequest<super::GetBlockHeaderByIdRequest>,
            ) -> Result<tonic::Response<super::BlockHeaderResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/flowrust.Api/GetBlockHeaderByID");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_block_header_by_height(
                &mut self,
                request: impl tonic::IntoRequest<super::GetBlockHeaderByHeightRequest>,
            ) -> Result<tonic::Response<super::BlockHeaderResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path =
                    http::uri::PathAndQuery::from_static("/flowrust.Api/GetBlockHeaderByHeight");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_latest_block(
                &mut self,
                request: impl tonic::IntoRequest<super::GetLatestBlockRequest>,
            ) -> Result<tonic::Response<super::BlockResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/flowrust.Api/GetLatestBlock");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_block_by_id(
                &mut self,
                request: impl tonic::IntoRequest<super::GetBlockByIdRequest>,
            ) -> Result<tonic::Response<super::BlockResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/flowrust.Api/GetBlockByID");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_block_by_height(
                &mut self,
                request: impl tonic::IntoRequest<super::GetBlockByHeightRequest>,
            ) -> Result<tonic::Response<super::BlockResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/flowrust.Api/GetBlockByHeight");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_collection_by_id(
                &mut self,
                request: impl tonic::IntoRequest<super::GetCollectionByIdRequest>,
            ) -> Result<tonic::Response<super::CollectionResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/flowrust.Api/GetCollectionByID");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn send_transaction(
                &mut self,
                request: impl tonic::IntoRequest<super::SendTransactionRequest>,
            ) -> Result<tonic::Response<super::SendTransactionResponse>, tonic::Status>
            {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/flowrust.Api/SendTransaction");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_transaction(
                &mut self,
                request: impl tonic::IntoRequest<super::GetTransactionRequest>,
            ) -> Result<tonic::Response<super::TransactionResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/flowrust.Api/GetTransaction");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_transaction_result(
                &mut self,
                request: impl tonic::IntoRequest<super::GetTransactionRequest>,
            ) -> Result<tonic::Response<super::TransactionResultResponse>, tonic::Status>
            {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path =
                    http::uri::PathAndQuery::from_static("/flowrust.Api/GetTransactionResult");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_account_at_latest_block(
                &mut self,
                request: impl tonic::IntoRequest<super::GetAccountAtLatestBlockRequest>,
            ) -> Result<tonic::Response<super::AccountResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path =
                    http::uri::PathAndQuery::from_static("/flowrust.Api/GetAccountAtLatestBlock");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_account_at_block_height(
                &mut self,
                request: impl tonic::IntoRequest<super::GetAccountAtBlockHeightRequest>,
            ) -> Result<tonic::Response<super::AccountResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path =
                    http::uri::PathAndQuery::from_static("/flowrust.Api/GetAccountAtBlockHeight");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn execute_script_at_latest_block(
                &mut self,
                request: impl tonic::IntoRequest<super::ExecuteScriptAtLatestBlockRequest>,
            ) -> Result<tonic::Response<super::ExecuteScriptResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/flowrust.Api/ExecuteScriptAtLatestBlock",
                );
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn execute_script_at_block_id(
                &mut self,
                request: impl tonic::IntoRequest<super::ExecuteScriptAtBlockIdRequest>,
            ) -> Result<tonic::Response<super::ExecuteScriptResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path =
                    http::uri::PathAndQuery::from_static("/flowrust.Api/ExecuteScriptAtBlockID");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn execute_script_at_block_height(
                &mut self,
                request: impl tonic::IntoRequest<super::ExecuteScriptAtBlockHeightRequest>,
            ) -> Result<tonic::Response<super::ExecuteScriptResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/flowrust.Api/ExecuteScriptAtBlockHeight",
                );
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_events_for_height_range(
                &mut self,
                request: impl tonic::IntoRequest<super::GetEventsForHeightRangeRequest>,
            ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path =
                    http::uri::PathAndQuery::from_static("/flowrust.Api/GetEventsForHeightRange");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_events_for_block_i_ds(
                &mut self,
                request: impl tonic::IntoRequest<super::GetEventsForBlockIdsRequest>,
            ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status> {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path =
                    http::uri::PathAndQuery::from_static("/flowrust.Api/GetEventsForBlockIDs");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_network_parameters(
                &mut self,
                request: impl tonic::IntoRequest<super::GetNetworkParametersRequest>,
            ) -> Result<tonic::Response<super::GetNetworkParametersResponse>, tonic::Status>
            {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path =
                    http::uri::PathAndQuery::from_static("/flowrust.Api/GetNetworkParameters");
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_latest_protocol_state_snapshot(
                &mut self,
                request: impl tonic::IntoRequest<super::GetLatestProtocolStateSnapshotRequest>,
            ) -> Result<tonic::Response<super::ProtocolStateSnapshotResponse>, tonic::Status>
            {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/flowrust.Api/GetLatestProtocolStateSnapshot",
                );
                self.inner.unary(request.into_request(), path, codec).await
            }
            pub async fn get_execution_result_for_block_id(
                &mut self,
                request: impl tonic::IntoRequest<super::GetExecutionResultForBlockIdRequest>,
            ) -> Result<tonic::Response<super::ExecutionResultForBlockIdResponse>, tonic::Status>
            {
                self.inner.ready().await.map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/flowrust.Api/GetExecutionResultForBlockID",
                );
                self.inner.unary(request.into_request(), path, codec).await
            }
        }
    }
    #[doc = r" Generated server implementations."]
    pub mod api_server {
        #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
        use tonic::codegen::*;
        #[doc = "Generated trait containing gRPC methods that should be implemented for use with ApiServer."]
        #[async_trait]
        pub trait Api: Send + Sync + 'static {
            #[doc = " the following is copied from docs.onflow.org/access-api/"]
            async fn ping(
                &self,
                request: tonic::Request<super::PingRequest>,
            ) -> Result<tonic::Response<super::PingResponse>, tonic::Status>;
            async fn get_latest_block_header(
                &self,
                request: tonic::Request<super::GetLatestBlockHeaderRequest>,
            ) -> Result<tonic::Response<super::BlockHeaderResponse>, tonic::Status>;
            async fn get_block_header_by_id(
                &self,
                request: tonic::Request<super::GetBlockHeaderByIdRequest>,
            ) -> Result<tonic::Response<super::BlockHeaderResponse>, tonic::Status>;
            async fn get_block_header_by_height(
                &self,
                request: tonic::Request<super::GetBlockHeaderByHeightRequest>,
            ) -> Result<tonic::Response<super::BlockHeaderResponse>, tonic::Status>;
            async fn get_latest_block(
                &self,
                request: tonic::Request<super::GetLatestBlockRequest>,
            ) -> Result<tonic::Response<super::BlockResponse>, tonic::Status>;
            async fn get_block_by_id(
                &self,
                request: tonic::Request<super::GetBlockByIdRequest>,
            ) -> Result<tonic::Response<super::BlockResponse>, tonic::Status>;
            async fn get_block_by_height(
                &self,
                request: tonic::Request<super::GetBlockByHeightRequest>,
            ) -> Result<tonic::Response<super::BlockResponse>, tonic::Status>;
            async fn get_collection_by_id(
                &self,
                request: tonic::Request<super::GetCollectionByIdRequest>,
            ) -> Result<tonic::Response<super::CollectionResponse>, tonic::Status>;
            async fn send_transaction(
                &self,
                request: tonic::Request<super::SendTransactionRequest>,
            ) -> Result<tonic::Response<super::SendTransactionResponse>, tonic::Status>;
            async fn get_transaction(
                &self,
                request: tonic::Request<super::GetTransactionRequest>,
            ) -> Result<tonic::Response<super::TransactionResponse>, tonic::Status>;
            async fn get_transaction_result(
                &self,
                request: tonic::Request<super::GetTransactionRequest>,
            ) -> Result<tonic::Response<super::TransactionResultResponse>, tonic::Status>;
            async fn get_account_at_latest_block(
                &self,
                request: tonic::Request<super::GetAccountAtLatestBlockRequest>,
            ) -> Result<tonic::Response<super::AccountResponse>, tonic::Status>;
            async fn get_account_at_block_height(
                &self,
                request: tonic::Request<super::GetAccountAtBlockHeightRequest>,
            ) -> Result<tonic::Response<super::AccountResponse>, tonic::Status>;
            async fn execute_script_at_latest_block(
                &self,
                request: tonic::Request<super::ExecuteScriptAtLatestBlockRequest>,
            ) -> Result<tonic::Response<super::ExecuteScriptResponse>, tonic::Status>;
            async fn execute_script_at_block_id(
                &self,
                request: tonic::Request<super::ExecuteScriptAtBlockIdRequest>,
            ) -> Result<tonic::Response<super::ExecuteScriptResponse>, tonic::Status>;
            async fn execute_script_at_block_height(
                &self,
                request: tonic::Request<super::ExecuteScriptAtBlockHeightRequest>,
            ) -> Result<tonic::Response<super::ExecuteScriptResponse>, tonic::Status>;
            async fn get_events_for_height_range(
                &self,
                request: tonic::Request<super::GetEventsForHeightRangeRequest>,
            ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status>;
            async fn get_events_for_block_i_ds(
                &self,
                request: tonic::Request<super::GetEventsForBlockIdsRequest>,
            ) -> Result<tonic::Response<super::EventsResponse>, tonic::Status>;
            async fn get_network_parameters(
                &self,
                request: tonic::Request<super::GetNetworkParametersRequest>,
            ) -> Result<tonic::Response<super::GetNetworkParametersResponse>, tonic::Status>;
            async fn get_latest_protocol_state_snapshot(
                &self,
                request: tonic::Request<super::GetLatestProtocolStateSnapshotRequest>,
            ) -> Result<tonic::Response<super::ProtocolStateSnapshotResponse>, tonic::Status>;
            async fn get_execution_result_for_block_id(
                &self,
                request: tonic::Request<super::GetExecutionResultForBlockIdRequest>,
            ) -> Result<tonic::Response<super::ExecutionResultForBlockIdResponse>, tonic::Status>;
        }
        #[derive(Debug)]
        pub struct ApiServer<T: Api> {
            inner: _Inner<T>,
            accept_compression_encodings: (),
            send_compression_encodings: (),
        }
        struct _Inner<T>(Arc<T>);
        impl<T: Api> ApiServer<T> {
            pub fn new(inner: T) -> Self {
                let inner = Arc::new(inner);
                let inner = _Inner(inner);
                Self {
                    inner,
                    accept_compression_encodings: Default::default(),
                    send_compression_encodings: Default::default(),
                }
            }
            pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
            {
                InterceptedService::new(Self::new(inner), interceptor)
            }
        }
        impl<T, B> tonic::codegen::Service<http::Request<B>> for ApiServer<T>
        where
            T: Api,
            B: Body + Send + Sync + 'static,
            B::Error: Into<StdError> + Send + 'static,
        {
            type Response = http::Response<tonic::body::BoxBody>;
            type Error = Never;
            type Future = BoxFuture<Self::Response, Self::Error>;
            fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: http::Request<B>) -> Self::Future {
                let inner = self.inner.clone();
                match req.uri().path() {
                    "/flowrust.Api/Ping" => {
                        #[allow(non_camel_case_types)]
                        struct PingSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::PingRequest> for PingSvc<T> {
                            type Response = super::PingResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::PingRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move { (*inner).ping(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = PingSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetLatestBlockHeader" => {
                        #[allow(non_camel_case_types)]
                        struct GetLatestBlockHeaderSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetLatestBlockHeaderRequest>
                            for GetLatestBlockHeaderSvc<T>
                        {
                            type Response = super::BlockHeaderResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetLatestBlockHeaderRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut =
                                    async move { (*inner).get_latest_block_header(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetLatestBlockHeaderSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetBlockHeaderByID" => {
                        #[allow(non_camel_case_types)]
                        struct GetBlockHeaderByIDSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetBlockHeaderByIdRequest>
                            for GetBlockHeaderByIDSvc<T>
                        {
                            type Response = super::BlockHeaderResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetBlockHeaderByIdRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut =
                                    async move { (*inner).get_block_header_by_id(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetBlockHeaderByIDSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetBlockHeaderByHeight" => {
                        #[allow(non_camel_case_types)]
                        struct GetBlockHeaderByHeightSvc<T: Api>(pub Arc<T>);
                        impl<T: Api>
                            tonic::server::UnaryService<super::GetBlockHeaderByHeightRequest>
                            for GetBlockHeaderByHeightSvc<T>
                        {
                            type Response = super::BlockHeaderResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetBlockHeaderByHeightRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).get_block_header_by_height(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetBlockHeaderByHeightSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetLatestBlock" => {
                        #[allow(non_camel_case_types)]
                        struct GetLatestBlockSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetLatestBlockRequest> for GetLatestBlockSvc<T> {
                            type Response = super::BlockResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetLatestBlockRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move { (*inner).get_latest_block(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetLatestBlockSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetBlockByID" => {
                        #[allow(non_camel_case_types)]
                        struct GetBlockByIDSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetBlockByIdRequest> for GetBlockByIDSvc<T> {
                            type Response = super::BlockResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetBlockByIdRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move { (*inner).get_block_by_id(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetBlockByIDSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetBlockByHeight" => {
                        #[allow(non_camel_case_types)]
                        struct GetBlockByHeightSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetBlockByHeightRequest>
                            for GetBlockByHeightSvc<T>
                        {
                            type Response = super::BlockResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetBlockByHeightRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut =
                                    async move { (*inner).get_block_by_height(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetBlockByHeightSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetCollectionByID" => {
                        #[allow(non_camel_case_types)]
                        struct GetCollectionByIDSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetCollectionByIdRequest>
                            for GetCollectionByIDSvc<T>
                        {
                            type Response = super::CollectionResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetCollectionByIdRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut =
                                    async move { (*inner).get_collection_by_id(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetCollectionByIDSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/SendTransaction" => {
                        #[allow(non_camel_case_types)]
                        struct SendTransactionSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::SendTransactionRequest> for SendTransactionSvc<T> {
                            type Response = super::SendTransactionResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::SendTransactionRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move { (*inner).send_transaction(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = SendTransactionSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetTransaction" => {
                        #[allow(non_camel_case_types)]
                        struct GetTransactionSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetTransactionRequest> for GetTransactionSvc<T> {
                            type Response = super::TransactionResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetTransactionRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move { (*inner).get_transaction(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetTransactionSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetTransactionResult" => {
                        #[allow(non_camel_case_types)]
                        struct GetTransactionResultSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetTransactionRequest>
                            for GetTransactionResultSvc<T>
                        {
                            type Response = super::TransactionResultResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetTransactionRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut =
                                    async move { (*inner).get_transaction_result(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetTransactionResultSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetAccountAtLatestBlock" => {
                        #[allow(non_camel_case_types)]
                        struct GetAccountAtLatestBlockSvc<T: Api>(pub Arc<T>);
                        impl<T: Api>
                            tonic::server::UnaryService<super::GetAccountAtLatestBlockRequest>
                            for GetAccountAtLatestBlockSvc<T>
                        {
                            type Response = super::AccountResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetAccountAtLatestBlockRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).get_account_at_latest_block(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetAccountAtLatestBlockSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetAccountAtBlockHeight" => {
                        #[allow(non_camel_case_types)]
                        struct GetAccountAtBlockHeightSvc<T: Api>(pub Arc<T>);
                        impl<T: Api>
                            tonic::server::UnaryService<super::GetAccountAtBlockHeightRequest>
                            for GetAccountAtBlockHeightSvc<T>
                        {
                            type Response = super::AccountResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetAccountAtBlockHeightRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).get_account_at_block_height(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetAccountAtBlockHeightSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/ExecuteScriptAtLatestBlock" => {
                        #[allow(non_camel_case_types)]
                        struct ExecuteScriptAtLatestBlockSvc<T: Api>(pub Arc<T>);
                        impl<T: Api>
                            tonic::server::UnaryService<super::ExecuteScriptAtLatestBlockRequest>
                            for ExecuteScriptAtLatestBlockSvc<T>
                        {
                            type Response = super::ExecuteScriptResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::ExecuteScriptAtLatestBlockRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).execute_script_at_latest_block(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = ExecuteScriptAtLatestBlockSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/ExecuteScriptAtBlockID" => {
                        #[allow(non_camel_case_types)]
                        struct ExecuteScriptAtBlockIDSvc<T: Api>(pub Arc<T>);
                        impl<T: Api>
                            tonic::server::UnaryService<super::ExecuteScriptAtBlockIdRequest>
                            for ExecuteScriptAtBlockIDSvc<T>
                        {
                            type Response = super::ExecuteScriptResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::ExecuteScriptAtBlockIdRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).execute_script_at_block_id(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = ExecuteScriptAtBlockIDSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/ExecuteScriptAtBlockHeight" => {
                        #[allow(non_camel_case_types)]
                        struct ExecuteScriptAtBlockHeightSvc<T: Api>(pub Arc<T>);
                        impl<T: Api>
                            tonic::server::UnaryService<super::ExecuteScriptAtBlockHeightRequest>
                            for ExecuteScriptAtBlockHeightSvc<T>
                        {
                            type Response = super::ExecuteScriptResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::ExecuteScriptAtBlockHeightRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).execute_script_at_block_height(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = ExecuteScriptAtBlockHeightSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetEventsForHeightRange" => {
                        #[allow(non_camel_case_types)]
                        struct GetEventsForHeightRangeSvc<T: Api>(pub Arc<T>);
                        impl<T: Api>
                            tonic::server::UnaryService<super::GetEventsForHeightRangeRequest>
                            for GetEventsForHeightRangeSvc<T>
                        {
                            type Response = super::EventsResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetEventsForHeightRangeRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).get_events_for_height_range(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetEventsForHeightRangeSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetEventsForBlockIDs" => {
                        #[allow(non_camel_case_types)]
                        struct GetEventsForBlockIDsSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetEventsForBlockIdsRequest>
                            for GetEventsForBlockIDsSvc<T>
                        {
                            type Response = super::EventsResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetEventsForBlockIdsRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).get_events_for_block_i_ds(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetEventsForBlockIDsSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetNetworkParameters" => {
                        #[allow(non_camel_case_types)]
                        struct GetNetworkParametersSvc<T: Api>(pub Arc<T>);
                        impl<T: Api> tonic::server::UnaryService<super::GetNetworkParametersRequest>
                            for GetNetworkParametersSvc<T>
                        {
                            type Response = super::GetNetworkParametersResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetNetworkParametersRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut =
                                    async move { (*inner).get_network_parameters(request).await };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetNetworkParametersSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetLatestProtocolStateSnapshot" => {
                        #[allow(non_camel_case_types)]
                        struct GetLatestProtocolStateSnapshotSvc<T: Api>(pub Arc<T>);
                        impl<T: Api>
                            tonic::server::UnaryService<
                                super::GetLatestProtocolStateSnapshotRequest,
                            > for GetLatestProtocolStateSnapshotSvc<T>
                        {
                            type Response = super::ProtocolStateSnapshotResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<
                                    super::GetLatestProtocolStateSnapshotRequest,
                                >,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).get_latest_protocol_state_snapshot(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetLatestProtocolStateSnapshotSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/flowrust.Api/GetExecutionResultForBlockID" => {
                        #[allow(non_camel_case_types)]
                        struct GetExecutionResultForBlockIDSvc<T: Api>(pub Arc<T>);
                        impl<T: Api>
                            tonic::server::UnaryService<super::GetExecutionResultForBlockIdRequest>
                            for GetExecutionResultForBlockIDSvc<T>
                        {
                            type Response = super::ExecutionResultForBlockIdResponse;
                            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::GetExecutionResultForBlockIdRequest>,
                            ) -> Self::Future {
                                let inner = self.0.clone();
                                let fut = async move {
                                    (*inner).get_execution_result_for_block_id(request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self.accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let inner = inner.0;
                            let method = GetExecutionResultForBlockIDSvc(inner);
                            let codec = tonic::codec::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    _ => Box::pin(async move {
                        Ok(http::Response::builder()
                            .status(200)
                            .header("grpc-status", "12")
                            .header("content-type", "application/grpc")
                            .body(empty_body())
                            .unwrap())
                    }),
                }
            }
        }
        impl<T: Api> Clone for ApiServer<T> {
            fn clone(&self) -> Self {
                let inner = self.inner.clone();
                Self {
                    inner,
                    accept_compression_encodings: self.accept_compression_encodings,
                    send_compression_encodings: self.send_compression_encodings,
                }
            }
        }
        impl<T: Api> Clone for _Inner<T> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
        impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
        impl<T: Api> tonic::transport::NamedService for ApiServer<T> {
            const NAME: &'static str = "flowrust.Api";
        }
    }
}

// ****************************************************
// Public Methods
// ****************************************************

// check the availability of the node at network_address
// if this times out, it's probably because the endpoint timed out.
pub async fn check_availability(network_address: String) -> Result<(), Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;

    let request = tonic::Request::new(PingRequest {});

    client.ping(request).await?;

    Ok(())
}

// get_account expects the address and will return the Account or an Err
pub async fn get_account(
    network_address: String,
    address: String,
) -> Result<AccountResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;

    let request = tonic::Request::new(GetAccountAtLatestBlockRequest {
        address: address.as_bytes().to_vec(),
    });

    let response = client.get_account_at_latest_block(request).await?;

    Ok(response.into_inner())
}

// execute_script will attempt to run the script and return the result as T or Error
pub async fn execute_script(
    network_address: String,
    script: Vec<u8>,
) -> Result<ExecuteScriptResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;

    let request = tonic::Request::new(ExecuteScriptAtLatestBlockRequest { script });

    let response = client.execute_script_at_latest_block(request).await?;

    Ok(response.into_inner())
}

// build
pub async fn build_transaction(
    script: Vec<u8>,
    arguments: Vec<Vec<u8>>,
    reference_block_id: Vec<u8>,
    gas_limit: u64,
    proposer: TransactionProposalKey,
    authorizers: Vec<Vec<u8>>,
    payer: Vec<u8>,
) -> Result<Transaction, Box<dyn error::Error>> {
    Ok(Transaction {
        script,
        arguments: arguments,
        reference_block_id: reference_block_id,
        gas_limit: gas_limit,
        proposal_key: Some(proposer),
        authorizers: authorizers,
        payload_signatures: vec![],
        envelope_signatures: vec![],
        payer: payer,
    })
}

// sign
pub async fn sign_transaction(
    built_transaction: Transaction,
    payload_signatures: Vec<TransactionSignature>,
    envelope_signatures: Vec<TransactionSignature>,
) -> Result<Option<Transaction>, Box<dyn error::Error>> {
    let signed_transaction = Some(Transaction {
        script: built_transaction.script,
        arguments: built_transaction.arguments,
        reference_block_id: built_transaction.reference_block_id,
        gas_limit: built_transaction.gas_limit,
        proposal_key: built_transaction.proposal_key,
        authorizers: built_transaction.authorizers,
        payload_signatures: payload_signatures,
        envelope_signatures: envelope_signatures,
        payer: built_transaction.payer,
    });
    Ok(signed_transaction)
}

// execute transaction
pub async fn execute_transaction(
    network_address: String,
    transaction: Option<Transaction>,
) -> Result<SendTransactionResponse, Box<dyn error::Error>> {
    // send to blockchain
    let mut client = ApiClient::connect(network_address).await?;

    let request = tonic::Request::new(SendTransactionRequest { transaction });

    let response = client.send_transaction(request).await?;

    Ok(response.into_inner())
}

// get_block accepts either the block_id or block_height. If neither are defined it returns the latest block.
pub async fn get_block(
    network_address: String,
    block_id: Option<String>,
    block_height: Option<u64>,
    is_sealed: Option<bool>,
) -> Result<BlockResponse, Box<dyn error::Error>> {
    if block_id.is_some() {
        // IF block_id, use this
        let mut client = ApiClient::connect(network_address).await?;
        let request = tonic::Request::new(GetBlockByIdRequest {
            id: block_id.unwrap().as_bytes().to_vec(),
        });
        let response = client.get_block_by_id(request).await?;
        Ok(response.into_inner())
    } else if block_height.is_some() {
        // else IF block_height, use that
        let mut client = ApiClient::connect(network_address).await?;
        let request = tonic::Request::new(GetBlockByHeightRequest {
            height: block_height.unwrap(),
        });
        let response = client.get_block_by_height(request).await?;
        Ok(response.into_inner())
    } else {
        // else, just get latest block
        if is_sealed.is_some() {
            let mut client = ApiClient::connect(network_address).await?;
            let request = tonic::Request::new(GetLatestBlockRequest {
                is_sealed: is_sealed.unwrap(),
            });
            let response = client.get_latest_block(request).await?;
            Ok(response.into_inner())
        } else {
            let mut client = ApiClient::connect(network_address).await?;
            let request = tonic::Request::new(GetLatestBlockRequest { is_sealed: true });
            let response = client.get_latest_block(request).await?;
            Ok(response.into_inner())
        }
    }
}

// retrieve the specified events by type for the given height range
pub async fn get_events_for_height_range(
    network_address: String,
    event_type: String,
    start_height: u64,
    end_height: u64,
) -> Result<EventsResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;
    let request = tonic::Request::new(GetEventsForHeightRangeRequest {
        r#type: event_type,
        start_height,
        end_height,
    });
    let response = client.get_events_for_height_range(request).await?;
    Ok(response.into_inner())
}

// retrieve the specified events by type for the given blocks
pub async fn get_events_for_block_ids(
    network_address: String,
    event_type: String,
    ids: Vec<Vec<u8>>,
) -> Result<EventsResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;
    let request = tonic::Request::new(GetEventsForBlockIdsRequest {
        r#type: event_type,
        block_ids: ids,
    });
    let response = client.get_events_for_block_i_ds(request).await?;
    Ok(response.into_inner())
}

// retrieve the specified collections
pub async fn get_collection(
    network_address: String,
    collection_id: Vec<u8>,
) -> Result<CollectionResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;
    let request = tonic::Request::new(GetCollectionByIdRequest { id: collection_id });
    let response = client.get_collection_by_id(request).await?;
    Ok(response.into_inner())
}

// ****************************************************
// Testing
// ****************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
