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
use std::sync::atomic::AtomicU64;

use crate::error::Error;
use crate::error::Result;

#[derive(Debug)]
pub struct S3_not_impl {
    pub(crate) root: PathBuf,
    tmp_file_counter: AtomicU64,
}

impl S3_not_impl {
    pub fn new(root: PathBuf) -> Result<Self> {
        if !root.is_dir() {
            return Err(Error::from_string(format!("{:?} is not a directory", root)));
        }
        Ok(Self {
            root,
            tmp_file_counter: AtomicU64::new(0),
        })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn tmp_file_counter(&self) -> u64 {
        self.tmp_file_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

#[async_trait::async_trait]
impl S3 for S3_not_impl {
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
