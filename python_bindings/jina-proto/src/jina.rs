///*
/// Represents a (quantized) dense n-dim array
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenseNdArrayProto {
    /// the actual array data, in bytes
    #[prost(bytes = "vec", tag = "1")]
    pub buffer: ::prost::alloc::vec::Vec<u8>,
    /// the shape (dimensions) of the array
    #[prost(uint32, repeated, tag = "2")]
    pub shape: ::prost::alloc::vec::Vec<u32>,
    /// the data type of the array
    #[prost(string, tag = "3")]
    pub dtype: ::prost::alloc::string::String,
    /// quantization mode
    #[prost(enumeration = "dense_nd_array_proto::QuantizationMode", tag = "4")]
    pub quantization: i32,
    /// the max value of the ndarray
    #[prost(float, tag = "5")]
    pub max_val: f32,
    /// the min value of the ndarray
    #[prost(float, tag = "6")]
    pub min_val: f32,
    /// the scale of the ndarray
    #[prost(float, tag = "7")]
    pub scale: f32,
    /// the original dtype of the array
    #[prost(string, tag = "8")]
    pub original_dtype: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DenseNdArrayProto`.
pub mod dense_nd_array_proto {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QuantizationMode {
        /// no quantization is performed, stored in the original ``dtype``
        None = 0,
        /// 2x smaller if dtype is set to FP32
        Fp16 = 1,
        /// 4x smaller but lossy when dtype is FP32
        Uint8 = 2,
    }
}
///*
/// Represents a general n-dim array, can be either dense or sparse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NdArrayProto {
    #[prost(oneof = "nd_array_proto::Content", tags = "1, 2")]
    pub content: ::core::option::Option<nd_array_proto::Content>,
}
/// Nested message and enum types in `NdArrayProto`.
pub mod nd_array_proto {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// dense representation of the ndarray
        #[prost(message, tag = "1")]
        Dense(super::DenseNdArrayProto),
        /// sparse representation of the ndarray
        #[prost(message, tag = "2")]
        Sparse(super::SparseNdArrayProto),
    }
}
///*
/// Represents a sparse ndarray
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparseNdArrayProto {
    /// A 2-D int64 tensor of shape [N, ndims], which specifies the indices of the elements in the sparse tensor that contain nonzero values (elements are zero-indexed)
    #[prost(message, optional, tag = "1")]
    pub indices: ::core::option::Option<DenseNdArrayProto>,
    /// A 1-D tensor of any type and shape [N], which supplies the values for each element in indices.
    #[prost(message, optional, tag = "2")]
    pub values: ::core::option::Option<DenseNdArrayProto>,
    /// A 1-D int64 tensor of shape [ndims], which specifies the dense_shape of the sparse tensor.
    #[prost(int64, repeated, tag = "3")]
    pub dense_shape: ::prost::alloc::vec::Vec<i64>,
}
///*
/// Represents the relevance model to `ref_id`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedScoreProto {
    /// value
    #[prost(float, tag = "1")]
    pub value: f32,
    /// the name of the operator/score function
    #[prost(string, tag = "2")]
    pub op_name: ::prost::alloc::string::String,
    /// text description of the score
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// the score can be nested
    #[prost(message, repeated, tag = "4")]
    pub operands: ::prost::alloc::vec::Vec<NamedScoreProto>,
    /// the score is computed between doc `id` and `ref_id`
    #[prost(string, tag = "5")]
    pub ref_id: ::prost::alloc::string::String,
}
///*
/// Represents a Document
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentProto {
    /// A hexdigest that represents a unique document ID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// A hexdigest that represents the hash of the content of the document
    #[prost(string, tag = "24")]
    pub content_hash: ::prost::alloc::string::String,
    /// the depth of the recursive chunk structure
    #[prost(uint32, tag = "14")]
    pub granularity: u32,
    /// the width of the recursive match structure
    #[prost(uint32, tag = "22")]
    pub adjacency: u32,
    /// the parent id from the previous granularity
    #[prost(string, tag = "16")]
    pub parent_id: ::prost::alloc::string::String,
    /// list of the sub-documents of this document (recursive structure)
    #[prost(message, repeated, tag = "4")]
    pub chunks: ::prost::alloc::vec::Vec<DocumentProto>,
    /// The weight of this document
    #[prost(float, tag = "5")]
    pub weight: f32,
    /// total number of siblings of this document (docs that are in the same granularity and have the same parent_id)
    #[prost(uint32, tag = "25")]
    pub siblings: u32,
    /// the matched documents on the same level (recursive structure)
    #[prost(message, repeated, tag = "8")]
    pub matches: ::prost::alloc::vec::Vec<DocumentProto>,
    /// mime type of this document, for buffer content, this is required; for other contents, this can be guessed
    #[prost(string, tag = "10")]
    pub mime_type: ::prost::alloc::string::String,
    /// a structured data value, consisting of field which map to dynamically typed values.
    #[prost(message, optional, tag = "11")]
    pub tags: ::core::option::Option<::prost_types::Struct>,
    /// the position of the doc, could be start and end index of a string; could be x,y (top, left) coordinate of an image crop; could be timestamp of an audio clip
    #[prost(uint32, repeated, tag = "17")]
    pub location: ::prost::alloc::vec::Vec<u32>,
    /// the offset of this doc in the previous granularity document
    #[prost(uint32, tag = "18")]
    pub offset: u32,
    /// the embedding `ndarray` of this document
    #[prost(message, optional, tag = "19")]
    pub embedding: ::core::option::Option<NdArrayProto>,
    /// TODO: List of matching scores performed on the document, each element corresponds to a metric
    #[prost(message, optional, tag = "20")]
    pub score: ::core::option::Option<NamedScoreProto>,
    /// modality, an identifier to the modality this document belongs to. In the scope of multi/cross modal search
    #[prost(string, tag = "21")]
    pub modality: ::prost::alloc::string::String,
    /// List of evaluations performed on the document, each element corresponds to a metric
    #[prost(message, repeated, tag = "23")]
    pub evaluations: ::prost::alloc::vec::Vec<NamedScoreProto>,
    #[prost(oneof = "document_proto::Content", tags = "3, 12, 13, 9")]
    pub content: ::core::option::Option<document_proto::Content>,
}
/// Nested message and enum types in `DocumentProto`.
pub mod document_proto {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// the raw binary content of this document, which often represents the original document when comes into jina
        #[prost(bytes, tag = "3")]
        Buffer(::prost::alloc::vec::Vec<u8>),
        /// the ndarray of the image/audio/video document
        #[prost(message, tag = "12")]
        Blob(super::NdArrayProto),
        /// a text document
        #[prost(string, tag = "13")]
        Text(::prost::alloc::string::String),
        /// a uri of the document could be: a local file path, a remote url starts with http or https or data URI scheme
        #[prost(string, tag = "9")]
        Uri(::prost::alloc::string::String),
    }
}
///*
/// Represents a the route paths of this message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteProto {
    /// the name of the BasePod
    #[prost(string, tag = "1")]
    pub pod: ::prost::alloc::string::String,
    /// the id of the BasePod
    #[prost(string, tag = "2")]
    pub pod_id: ::prost::alloc::string::String,
    /// receiving time
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// sending (out) time
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// the status of the execution
    #[prost(message, optional, tag = "5")]
    pub status: ::core::option::Option<StatusProto>,
}
///*
/// Represents a Envelope, a part of the ``Message``.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvelopeProto {
    /// unique id of the sender of the message
    #[prost(string, tag = "1")]
    pub sender_id: ::prost::alloc::string::String,
    /// unique id of the receiver of the message, only used in router-dealer pattern
    #[prost(string, tag = "2")]
    pub receiver_id: ::prost::alloc::string::String,
    /// unique id of the request
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// timeout in second until this message is dropped
    #[prost(uint32, tag = "4")]
    pub timeout: u32,
    /// version info
    #[prost(message, optional, tag = "6")]
    pub version: ::core::option::Option<envelope_proto::VersionProto>,
    /// type of the request: DataRequest, ControlRequest
    #[prost(string, tag = "7")]
    pub request_type: ::prost::alloc::string::String,
    /// check local Protobuf version on every Pod that this message flows to
    #[prost(bool, tag = "8")]
    pub check_version: bool,
    /// compress configuration used for request
    #[prost(message, optional, tag = "9")]
    pub compression: ::core::option::Option<envelope_proto::CompressConfigProto>,
    /// status info on every routes
    #[prost(message, repeated, tag = "10")]
    pub routes: ::prost::alloc::vec::Vec<RouteProto>,
    /// status info
    #[prost(message, optional, tag = "11")]
    pub status: ::core::option::Option<StatusProto>,
    /// header contains meta info defined by the user, copied from Request, for lazy serialization
    #[prost(message, optional, tag = "12")]
    pub header: ::core::option::Option<HeaderProto>,
}
/// Nested message and enum types in `EnvelopeProto`.
pub mod envelope_proto {
    ///*
    /// Represents a the version information
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VersionProto {
        /// jina's version
        #[prost(string, tag = "1")]
        pub jina: ::prost::alloc::string::String,
        /// protobuf's version
        #[prost(string, tag = "2")]
        pub proto: ::prost::alloc::string::String,
        /// vcs's version
        #[prost(string, tag = "3")]
        pub vcs: ::prost::alloc::string::String,
    }
    ///*
    /// Represents a config for the compression algorithm
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompressConfigProto {
        /// compress algorithm used for request
        #[prost(string, tag = "1")]
        pub algorithm: ::prost::alloc::string::String,
        /// the high watermark that triggers the message compression. message bigger than this HWM (in bytes) will be compressed by the algorithm.
        #[prost(uint64, tag = "2")]
        pub min_bytes: u64,
        /// the low watermark that enables the sending of a compressed message. compression rate (after_size/before_size) lower than this LWM will be considered as successeful compression, and will be sent. Otherwise, it will send the original message without compression
        #[prost(float, tag = "3")]
        pub min_ratio: f32,
        /// other parameters that can be accepted by the algorithm
        #[prost(message, optional, tag = "4")]
        pub parameters: ::core::option::Option<::prost_types::Struct>,
    }
}
///*
/// Represents a Header.
/// - The header's content will be defined by the user request.
/// - It will be copied to the envelope.header
/// - In-flow operations will modify the envelope.header
/// - While returning, copy envelope.header back to request.header
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderProto {
    /// the endpoint specified by `@requests(on='/abc')`
    #[prost(string, tag = "1")]
    pub exec_endpoint: ::prost::alloc::string::String,
    /// if set, the request is targeted to certain peas/pods, regex strings
    #[prost(string, tag = "2")]
    pub target_peapod: ::prost::alloc::string::String,
    /// if set, then this request is not propagate over the Flow topology
    #[prost(bool, tag = "3")]
    pub no_propagate: bool,
}
///*
/// Represents a Status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusProto {
    /// status code
    #[prost(enumeration = "status_proto::StatusCode", tag = "1")]
    pub code: i32,
    /// error description of the very first exception
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the details of the error
    #[prost(message, optional, tag = "3")]
    pub exception: ::core::option::Option<status_proto::ExceptionProto>,
}
/// Nested message and enum types in `StatusProto`.
pub mod status_proto {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExceptionProto {
        /// the class name of the exception
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// the list of arguments given to the exception constructor.
        #[prost(string, repeated, tag = "2")]
        pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// the exception traceback stacks
        #[prost(string, repeated, tag = "3")]
        pub stacks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// the name of the executor bind to that peapod (if applicable)
        #[prost(string, tag = "4")]
        pub executor: ::prost::alloc::string::String,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StatusCode {
        /// success
        Success = 0,
        /// there are pending messages, more messages are followed
        Pending = 1,
        /// ready to use
        Ready = 2,
        /// error
        Error = 3,
        /// already a existing pod running
        ErrorDuplicate = 4,
        /// not allowed to open pod remotely
        ErrorNotallowed = 5,
        /// chained from the previous error
        ErrorChained = 6,
    }
}
///*
/// Represents a Message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageProto {
    /// the envelope of the message, used internally in jina, dropped when returning to client
    #[prost(message, optional, tag = "1")]
    pub envelope: ::core::option::Option<EnvelopeProto>,
    /// the request body
    #[prost(message, optional, tag = "2")]
    pub request: ::core::option::Option<RequestProto>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentArrayProto {
    /// a list of Documents
    #[prost(message, repeated, tag = "1")]
    pub docs: ::prost::alloc::vec::Vec<DocumentProto>,
}
///*
/// Represents a Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestProto {
    /// the unique ID of this request. Multiple requests with the same ID will be gathered
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// header contains meta info defined by the user
    #[prost(message, optional, tag = "4")]
    pub header: ::core::option::Option<HeaderProto>,
    /// extra kwargs that will be used in executor
    #[prost(message, optional, tag = "5")]
    pub parameters: ::core::option::Option<::prost_types::Struct>,
    /// status info on every routes
    #[prost(message, repeated, tag = "6")]
    pub routes: ::prost::alloc::vec::Vec<RouteProto>,
    /// status info
    #[prost(message, optional, tag = "7")]
    pub status: ::core::option::Option<StatusProto>,
    #[prost(oneof = "request_proto::Body", tags = "2, 3")]
    pub body: ::core::option::Option<request_proto::Body>,
}
/// Nested message and enum types in `RequestProto`.
pub mod request_proto {
    ///*
    /// Represents a general data request
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataRequestProto {
        /// a list of Documents to query
        #[prost(message, repeated, tag = "1")]
        pub docs: ::prost::alloc::vec::Vec<super::DocumentProto>,
        /// a list of groundtruth Document you want to evaluate it with
        #[prost(message, repeated, tag = "2")]
        pub groundtruths: ::prost::alloc::vec::Vec<super::DocumentProto>,
    }
    ///*
    /// Represents a control request used to control the BasePod
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ControlRequestProto {
        /// the control command
        #[prost(enumeration = "control_request_proto::Command", tag = "1")]
        pub command: i32,
    }
    /// Nested message and enum types in `ControlRequestProto`.
    pub mod control_request_proto {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Command {
            /// shutdown the BasePod
            Terminate = 0,
            /// check the status of the BasePod
            Status = 1,
            /// used in ROUTER-DEALER pattern, tells the router that the dealer is idle
            Idle = 2,
            /// used in ROUTER-DEALER pattern, tells the router that the dealer is busy (or closed)
            Cancel = 3,
            /// scale up/down a Pod
            Scale = 4,
            /// used in ROUTER-DEALER pattern, Indicate a Pea that it can activate itself and send the IDLE command to their router
            Activate = 5,
            /// used in ROUTER-DEALER pattern, Indicate a Pea that it can deactivate itself and send the CANCEL command to their router
            Deactivate = 6,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Body {
        /// a control request
        #[prost(message, tag = "2")]
        Control(ControlRequestProto),
        /// a data request
        #[prost(message, tag = "3")]
        Data(DataRequestProto),
    }
}
#[doc = r" Generated client implementations."]
pub mod jina_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "*"]
    #[doc = " jina gRPC service."]
    pub struct JinaRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl JinaRpcClient<tonic::transport::Channel> {
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
    impl<T> JinaRpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Pass in a Request and a filled Request with matches will be returned."]
        pub async fn call(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::RequestProto>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::RequestProto>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/jina.JinaRPC/Call");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for JinaRpcClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for JinaRpcClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "JinaRpcClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod jina_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with JinaRpcServer."]
    #[async_trait]
    pub trait JinaRpc: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Call method."]
        type CallStream: futures_core::Stream<Item = Result<super::RequestProto, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Pass in a Request and a filled Request with matches will be returned."]
        async fn call(
            &self,
            request: tonic::Request<tonic::Streaming<super::RequestProto>>,
        ) -> Result<tonic::Response<Self::CallStream>, tonic::Status>;
    }
    #[doc = "*"]
    #[doc = " jina gRPC service."]
    #[derive(Debug)]
    pub struct JinaRpcServer<T: JinaRpc> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: JinaRpc> JinaRpcServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for JinaRpcServer<T>
    where
        T: JinaRpc,
        B: HttpBody + Send + Sync + 'static,
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
                "/jina.JinaRPC/Call" => {
                    #[allow(non_camel_case_types)]
                    struct CallSvc<T: JinaRpc>(pub Arc<T>);
                    impl<T: JinaRpc> tonic::server::StreamingService<super::RequestProto> for CallSvc<T> {
                        type Response = super::RequestProto;
                        type ResponseStream = T::CallStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::RequestProto>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).call(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = CallSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: JinaRpc> Clone for JinaRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: JinaRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: JinaRpc> tonic::transport::NamedService for JinaRpcServer<T> {
        const NAME: &'static str = "jina.JinaRPC";
    }
}
