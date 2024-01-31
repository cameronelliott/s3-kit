#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

// S3DEC
// Dec stands for distributed erasure coding.mod s3_btree;

use s3s::dto::GetObjectInput;
use s3s::dto::GetObjectOutput;
use s3s::dto::*;

use s3s::S3Result;

use s3s::S3;
use s3s::{S3Request, S3Response};

use crate::error::Result;

use crate::s3_btree::S3Btree;

#[derive(Debug)]
pub struct S3ToHttp;

#[async_trait::async_trait]
impl S3 for S3ToHttp {}

//
//
//

#[derive(Debug)]
pub struct Proxy<T: S3> {
    target: T,
}

impl Proxy<S3Btree> {
    #[allow(dead_code)]
    pub fn new() -> Result<Self> {
        Ok(Self {
            target: S3Btree::default(),
        })
    }
}

impl Proxy<S3ToHttp> {
    #[allow(dead_code)]
    pub fn new() -> Result<Self> {
        Ok(Self {
            target: S3ToHttp {},
        })
    }
}

#[async_trait::async_trait]
impl<T: S3 + std::fmt::Debug> S3 for Proxy<T> {
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
        self: &Proxy<T>,
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
        //println!("head_object: {:?}", req);
        self.target.head_object(req).await
        //
    }
}
