// This file is @generated by prost-build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Code {
    Ok = 0,
    Cancelled = 1,
    Unknown = 2,
    InvalidArgument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    Unauthenticated = 16,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
    DataLoss = 15,
}
impl Code {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Cancelled => "CANCELLED",
            Self::Unknown => "UNKNOWN",
            Self::InvalidArgument => "INVALID_ARGUMENT",
            Self::DeadlineExceeded => "DEADLINE_EXCEEDED",
            Self::NotFound => "NOT_FOUND",
            Self::AlreadyExists => "ALREADY_EXISTS",
            Self::PermissionDenied => "PERMISSION_DENIED",
            Self::Unauthenticated => "UNAUTHENTICATED",
            Self::ResourceExhausted => "RESOURCE_EXHAUSTED",
            Self::FailedPrecondition => "FAILED_PRECONDITION",
            Self::Aborted => "ABORTED",
            Self::OutOfRange => "OUT_OF_RANGE",
            Self::Unimplemented => "UNIMPLEMENTED",
            Self::Internal => "INTERNAL",
            Self::Unavailable => "UNAVAILABLE",
            Self::DataLoss => "DATA_LOSS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OK" => Some(Self::Ok),
            "CANCELLED" => Some(Self::Cancelled),
            "UNKNOWN" => Some(Self::Unknown),
            "INVALID_ARGUMENT" => Some(Self::InvalidArgument),
            "DEADLINE_EXCEEDED" => Some(Self::DeadlineExceeded),
            "NOT_FOUND" => Some(Self::NotFound),
            "ALREADY_EXISTS" => Some(Self::AlreadyExists),
            "PERMISSION_DENIED" => Some(Self::PermissionDenied),
            "UNAUTHENTICATED" => Some(Self::Unauthenticated),
            "RESOURCE_EXHAUSTED" => Some(Self::ResourceExhausted),
            "FAILED_PRECONDITION" => Some(Self::FailedPrecondition),
            "ABORTED" => Some(Self::Aborted),
            "OUT_OF_RANGE" => Some(Self::OutOfRange),
            "UNIMPLEMENTED" => Some(Self::Unimplemented),
            "INTERNAL" => Some(Self::Internal),
            "UNAVAILABLE" => Some(Self::Unavailable),
            "DATA_LOSS" => Some(Self::DataLoss),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorInfo {
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "3")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
impl ::prost::Name for ErrorInfo {
    const NAME: &'static str = "ErrorInfo";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.ErrorInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.ErrorInfo".into()
    }
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RetryInfo {
    #[prost(message, optional, tag = "1")]
    pub retry_delay: ::core::option::Option<::prost_types::Duration>,
}
impl ::prost::Name for RetryInfo {
    const NAME: &'static str = "RetryInfo";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.RetryInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.RetryInfo".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugInfo {
    #[prost(string, repeated, tag = "1")]
    pub stack_entries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub detail: ::prost::alloc::string::String,
}
impl ::prost::Name for DebugInfo {
    const NAME: &'static str = "DebugInfo";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.DebugInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.DebugInfo".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaFailure {
    #[prost(message, repeated, tag = "1")]
    pub violations: ::prost::alloc::vec::Vec<quota_failure::Violation>,
}
/// Nested message and enum types in `QuotaFailure`.
pub mod quota_failure {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Violation {
        #[prost(string, tag = "1")]
        pub subject: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub api_service: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub quota_metric: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub quota_id: ::prost::alloc::string::String,
        #[prost(map = "string, string", tag = "6")]
        pub quota_dimensions: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        #[prost(int64, tag = "7")]
        pub quota_value: i64,
        #[prost(int64, optional, tag = "8")]
        pub future_quota_value: ::core::option::Option<i64>,
    }
    impl ::prost::Name for Violation {
        const NAME: &'static str = "Violation";
        const PACKAGE: &'static str = "google.rpc";
        fn full_name() -> ::prost::alloc::string::String {
            "google.rpc.QuotaFailure.Violation".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/google.rpc.QuotaFailure.Violation".into()
        }
    }
}
impl ::prost::Name for QuotaFailure {
    const NAME: &'static str = "QuotaFailure";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.QuotaFailure".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.QuotaFailure".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreconditionFailure {
    #[prost(message, repeated, tag = "1")]
    pub violations: ::prost::alloc::vec::Vec<precondition_failure::Violation>,
}
/// Nested message and enum types in `PreconditionFailure`.
pub mod precondition_failure {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Violation {
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub subject: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub description: ::prost::alloc::string::String,
    }
    impl ::prost::Name for Violation {
        const NAME: &'static str = "Violation";
        const PACKAGE: &'static str = "google.rpc";
        fn full_name() -> ::prost::alloc::string::String {
            "google.rpc.PreconditionFailure.Violation".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/google.rpc.PreconditionFailure.Violation".into()
        }
    }
}
impl ::prost::Name for PreconditionFailure {
    const NAME: &'static str = "PreconditionFailure";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.PreconditionFailure".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.PreconditionFailure".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadRequest {
    #[prost(message, repeated, tag = "1")]
    pub field_violations: ::prost::alloc::vec::Vec<bad_request::FieldViolation>,
}
/// Nested message and enum types in `BadRequest`.
pub mod bad_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldViolation {
        #[prost(string, tag = "1")]
        pub field: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub reason: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "4")]
        pub localized_message: ::core::option::Option<super::LocalizedMessage>,
    }
    impl ::prost::Name for FieldViolation {
        const NAME: &'static str = "FieldViolation";
        const PACKAGE: &'static str = "google.rpc";
        fn full_name() -> ::prost::alloc::string::String {
            "google.rpc.BadRequest.FieldViolation".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/google.rpc.BadRequest.FieldViolation".into()
        }
    }
}
impl ::prost::Name for BadRequest {
    const NAME: &'static str = "BadRequest";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.BadRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.BadRequest".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInfo {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub serving_data: ::prost::alloc::string::String,
}
impl ::prost::Name for RequestInfo {
    const NAME: &'static str = "RequestInfo";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.RequestInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.RequestInfo".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceInfo {
    #[prost(string, tag = "1")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
impl ::prost::Name for ResourceInfo {
    const NAME: &'static str = "ResourceInfo";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.ResourceInfo".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.ResourceInfo".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Help {
    #[prost(message, repeated, tag = "1")]
    pub links: ::prost::alloc::vec::Vec<help::Link>,
}
/// Nested message and enum types in `Help`.
pub mod help {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Link {
        #[prost(string, tag = "1")]
        pub description: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub url: ::prost::alloc::string::String,
    }
    impl ::prost::Name for Link {
        const NAME: &'static str = "Link";
        const PACKAGE: &'static str = "google.rpc";
        fn full_name() -> ::prost::alloc::string::String {
            "google.rpc.Help.Link".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/google.rpc.Help.Link".into()
        }
    }
}
impl ::prost::Name for Help {
    const NAME: &'static str = "Help";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.Help".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.Help".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalizedMessage {
    #[prost(string, tag = "1")]
    pub locale: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for LocalizedMessage {
    const NAME: &'static str = "LocalizedMessage";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.LocalizedMessage".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.LocalizedMessage".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequest {
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub headers: ::prost::alloc::vec::Vec<HttpHeader>,
    #[prost(bytes = "bytes", tag = "4")]
    pub body: ::prost::bytes::Bytes,
}
impl ::prost::Name for HttpRequest {
    const NAME: &'static str = "HttpRequest";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.HttpRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.HttpRequest".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpResponse {
    #[prost(int32, tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub headers: ::prost::alloc::vec::Vec<HttpHeader>,
    #[prost(bytes = "bytes", tag = "4")]
    pub body: ::prost::bytes::Bytes,
}
impl ::prost::Name for HttpResponse {
    const NAME: &'static str = "HttpResponse";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.HttpResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.HttpResponse".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpHeader {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
impl ::prost::Name for HttpHeader {
    const NAME: &'static str = "HttpHeader";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.HttpHeader".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.HttpHeader".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub details: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
impl ::prost::Name for Status {
    const NAME: &'static str = "Status";
    const PACKAGE: &'static str = "google.rpc";
    fn full_name() -> ::prost::alloc::string::String {
        "google.rpc.Status".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/google.rpc.Status".into()
    }
}
