#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Amazon EMR Serverless is a new deployment option for Amazon EMR. EMR Serverless provides
//! a serverless runtime environment that simplifies running analytics applications using the
//! latest open source frameworks such as Apache Spark and Apache Hive. With EMR Serverless,
//! you don’t have to configure, optimize, secure, or operate clusters to run applications with
//! these frameworks.</p>
//! <p>The API reference to Amazon EMR Serverless is <code>emr-serverless</code>. The
//! <code>emr-serverless</code> prefix is used in the following scenarios: </p>
//! <ul>
//! <li>
//! <p>It is the prefix in the CLI commands for Amazon EMR Serverless. For example,
//! <code>aws emr-serverless start-job-run</code>.</p>
//! </li>
//! <li>
//! <p>It is the prefix before IAM policy actions for Amazon EMR Serverless. For example,
//! <code>"Action": ["emr-serverless:StartJobRun"]</code>. For more information, see
//! <a href="https://docs.aws.amazon.com/emr/latest/EMR-Serverless-UserGuide/security_iam_service-with-iam.html#security_iam_service-with-iam-id-based-policies-actions">Policy actions for Amazon EMR Serverless</a>.</p>
//! </li>
//! <li>
//! <p>It is the prefix used in Amazon EMR Serverless service endpoints. For example,
//! <code>emr-serverless.us-east-2.amazonaws.com</code>.</p>
//! </li>
//! </ul>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
mod idempotency_token;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
pub mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::DateTime;
}
pub use aws_smithy_types::retry::RetryConfig;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("emrserverless", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
