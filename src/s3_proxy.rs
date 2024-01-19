#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

// S3DEC
// Dec stands for distributed erasure coding.mod s3_btree;

use std::collections::BTreeMap;
use std::path::PathBuf;

use std::sync::Arc;

use std::time::SystemTime;

use bytes::Bytes;

use futures::TryStreamExt;
use hyper::service::Service;
use rust_utils::default::default;
use s3s::dto::GetObjectInput;
use s3s::dto::GetObjectOutput;
use s3s::dto::*;
use s3s::s3_error;

use s3s::S3Result;

use s3s::S3;
use s3s::{S3Request, S3Response};

use md5::Digest;
use md5::Md5;
use tokio::sync::RwLock;

use std::io::Cursor;
use tokio::io::BufWriter;

use crate::error::Error;
use crate::error::Result;

use crate::s3_btree::S3Btree;
use crate::utils::copy_bytes;
use crate::vec_byte_stream::VecByteStream;

use futures::Future;
/*
this is an interim test before writing the distributed erasure coding
*/

type XXX = dyn Service<
    S3Request<GetObjectInput>,
    Response = S3Result<S3Response<GetObjectOutput>>,
    Error = Error,
    Future = dyn Future<Output = Result<S3Result<S3Response<GetObjectOutput>>>> + Send,
>;

#[derive(Debug)]
pub struct S3ServiceProxy {
    target: Box<S3Btree>,
}

impl S3ServiceProxy {
    pub fn new() -> Result<Self> {
        Ok(Self {
            target: Box::new(S3Btree::new()?),
        })
    }
}

#[async_trait::async_trait]
impl S3 for S3ServiceProxy {
    #[tracing::instrument]
    async fn get_object(
        &self,
        req: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        //
        self.target.get_object(req).await
        //
    }

    async fn put_object(
        self: &S3ServiceProxy,
        req: S3Request<PutObjectInput>,
    ) -> S3Result<S3Response<PutObjectOutput>> {
        //
        self.target.put_object(req).await
        //
    }

    async fn delete_object(
        &self,
        req: S3Request<DeleteObjectInput>,
    ) -> S3Result<S3Response<DeleteObjectOutput>> {
        //
        self.target.delete_object(req).await
        //
    }

    #[tracing::instrument]
    async fn head_object(
        &self,
        req: S3Request<HeadObjectInput>,
    ) -> S3Result<S3Response<HeadObjectOutput>> {
        //
        self.target.head_object(req).await
        //
    }
}

// from s3s, s3s copyright stands
pub fn hex(input: impl AsRef<[u8]>) -> String {
    hex_simd::encode_to_string(input.as_ref(), hex_simd::AsciiCase::Lower)
}
