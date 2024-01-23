#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::collections::BTreeMap;

use std::sync::Arc;

use bytes::Bytes;

use futures::stream::FuturesUnordered;
use futures::StreamExt;

use derivative::Derivative;
use futures::TryStreamExt;
use rust_utils::default::default;
use s3s::dto::builders::HeadObjectInputBuilder;
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
use tracing::info;

use std::io::Cursor;
use tokio::io::BufWriter;

use crate::utils::copy_bytes;
use crate::vec_byte_stream::VecByteStream;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct S3Replication<T: S3 + Default> {
    objects: Arc<RwLock<BTreeMap<String, Vec<u8>>>>,
    #[derivative(Debug = "ignore")]
    //target: Box<dyn S3>,
    target1: T,
    target2: T,
}

// impl S3Replication {
//     pub fn new() -> Result<Self> {
//         let slf = Self {
//             objects: Arc::new(RwLock::new(BTreeMap::new())),
//             target1: Box::new(T::new()?),
//             target2: Box::new(S3Btree::new()?),
//         };
//         Ok(slf)
//     }
// }

// impl<T> Clone for S3Request<T>
// where
//     T: Clone,
// {
//     fn clone(&self) -> Self {
//         S3Request {
//             input: self.input.clone(),
//             credentials: self.credentials.clone(),
//             extensions: self.extensions.clone(),
//             headers: self.headers.clone(),
//             uri: self.uri.clone(),
//         }
//     }
// }

#[async_trait::async_trait]
impl<T: S3 + std::fmt::Debug + Default> S3 for S3Replication<T> {
    #[tracing::instrument]
    async fn get_object(
        &self,
        _req: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        // let x = req.extensions.

        //     let v2_resp = self.target1.get_object(req.map_input(Into::into)).await?;
        //     let v1_resp = self.target2.get_object(req.map_input(Into::into)).await?;

        // let a = self.target1.get_object(xx).await;
        // let b = self.target2.get_object(req).await;

        let len = 3;

        let foo = Bytes::from(b"wow".to_vec());
        let vec_stream = VecByteStream::new(vec![foo]);
        let streaming_blob = StreamingBlob::new(vec_stream);

        let output = GetObjectOutput {
            //  body: Some(StreamingBlob::wrap(streaming_blob)),
            body: Some(streaming_blob),
            //body: None,
            content_length: len,
            last_modified: None,
            metadata: None,
            e_tag: None,
            checksum_crc32: None,  //checksum.checksum_crc32,
            checksum_crc32c: None, // checksum.checksum_crc32c,
            checksum_sha1: None,   // checksum.checksum_sha1,
            checksum_sha256: None, // checksum.checksum_sha256,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }

    async fn put_object(
        self: &S3Replication<T>,
        req: S3Request<PutObjectInput>,
    ) -> S3Result<S3Response<PutObjectOutput>> {
        let input = req.input;
        if let Some(ref storage_class) = input.storage_class {
            let is_valid = ["STANDARD", "REDUCED_REDUNDANCY"].contains(&storage_class.as_str());
            if !is_valid {
                return Err(s3_error!(InvalidStorageClass));
            }
        }

        let PutObjectInput { body, key, .. } = input;

        let Some(body) = body else {
            return Err(s3_error!(IncompleteBody));
        };

        let mut checksum: crate::checksum::ChecksumCalculator = default();
        if input.checksum_crc32.is_some() {
            checksum.crc32 = Some(default());
        }
        if input.checksum_crc32c.is_some() {
            checksum.crc32c = Some(default());
        }
        if input.checksum_sha1.is_some() {
            checksum.sha1 = Some(default());
        }
        if input.checksum_sha256.is_some() {
            checksum.sha256 = Some(default());
        }

        let mut md5_hash = Md5::new();
        let stream = body.inspect_ok(|bytes| {
            md5_hash.update(bytes.as_ref());
            checksum.update(bytes.as_ref());
        });

        let buffer = Vec::new();
        let mut writer = BufWriter::new(Cursor::new(buffer));

        let _size = copy_bytes(stream, &mut writer).await?;

        let vec = writer.into_inner().into_inner();

        let mut objects = self.objects.write().await;
        objects.insert(key, vec.to_owned());

        return Ok(S3Response::new(PutObjectOutput::default()));
    }

    async fn delete_object(
        &self,
        req: S3Request<DeleteObjectInput>,
    ) -> S3Result<S3Response<DeleteObjectOutput>> {
        let input = req.input;

        let DeleteObjectInput { key, .. } = input;

        if self.objects.write().await.remove(&key).is_none() {
            return Err(s3_error!(NoSuchKey));
        }

        return Ok(S3Response::new(DeleteObjectOutput::default()));
    }

    #[tracing::instrument]
    async fn head_object(
        &self,
        _r: S3Request<HeadObjectInput>,
    ) -> S3Result<S3Response<HeadObjectOutput>> {
        //
        // let mut set = JoinSet::new();

        let b = HeadObjectInputBuilder::default()
            .bucket("".to_string())
            .key("".to_string())
            .build()
            .unwrap();

        let f1 = self.target1.head_object(S3Request::new(b));

        let b = HeadObjectInputBuilder::default()
            .bucket("".to_string())
            .key("".to_string())
            .build()
            .unwrap();

        let f2 = self.target1.head_object(S3Request::new(b));

        let mut futures = FuturesUnordered::new();

        futures.push(f1);
        futures.push(f2);

        while let Some(result) = futures.next().await {
            // Handle result
            info!("Got result: {}", result.is_ok());
        }

        let len = 3;

        // TODO: detect content type
        let content_type = mime::APPLICATION_OCTET_STREAM;

        let output = HeadObjectOutput {
            content_length: try_!(i64::try_from(len)),
            content_type: Some(content_type),
            last_modified: None,
            metadata: None,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }
}

// from s3s, s3s copyright stands
pub fn hex(input: impl AsRef<[u8]>) -> String {
    hex_simd::encode_to_string(input.as_ref(), hex_simd::AsciiCase::Lower)
}
