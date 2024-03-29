//#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

#[macro_use]
pub mod error;

pub mod checksum;
pub mod s3_btree;
pub mod s3_dec;
pub mod s3_not_impl;
pub mod s3_proxy_example;
pub mod s3_replication;
pub mod utils;
pub mod vec_byte_stream;

pub use error::Error;
pub use error::Result;

pub mod tracing;

pub mod fuzzing {
    use libfuzzer_sys::arbitrary::Arbitrary;

    #[derive(Debug, Arbitrary, bitcode::Encode, bitcode::Decode)]
    pub struct BackendS3Instructions {
        pub real_bucket: String,
        pub rand_fail: bool,
    }
}

pub mod fix_await_coverage;
