#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use s3s::dto::GetObjectInput;
use s3s::dto::GetObjectOutput;
use s3s::dto::*;
use s3s::s3_error;

use s3s::S3Result;
use s3s::S3;
use s3s::{S3Request, S3Response};

use std::path::PathBuf;

use crate::error::Result;

#[derive(Debug)]
pub struct S3NotImpl {}

impl S3NotImpl {
    #[allow(dead_code)]
    pub fn new(_root: PathBuf) -> Result<Self> {
        Ok(Self {})
    }
}

#[async_trait::async_trait]
impl S3 for S3NotImpl {
    #[tracing::instrument]
    async fn get_object(
        &self,
        _req: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        return Err(s3_error!(NoSuchKey));
    }

    async fn put_object(
        &self,
        _req: S3Request<PutObjectInput>,
    ) -> S3Result<S3Response<PutObjectOutput>> {
        return Err(s3_error!(NoSuchKey));
    }

    async fn delete_object(
        &self,
        _req: S3Request<DeleteObjectInput>,
    ) -> S3Result<S3Response<DeleteObjectOutput>> {
        return Err(s3_error!(NoSuchKey));
    }
}
