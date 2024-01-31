#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::collections::BTreeMap;

use std::sync::Arc;

use std::time::SystemTime;

use bytes::Bytes;

use futures::TryStreamExt;
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

use crate::error::Result;

use crate::utils::copy_bytes;
use crate::vec_byte_stream::VecByteStream;

#[derive(Debug)]
struct State {
    btree: BTreeMap<String, Vec<u8>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            btree: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct S3Btree {
    objects: Arc<RwLock<State>>,
    fake_rand: bool,
}

impl Default for S3Btree {
    fn default() -> Self {
        Self {
            objects: Arc::new(RwLock::new(State::default())), 
            fake_rand: false,
        }
    }
}

// impl S3Btree {
//     pub fn new() -> Result<Self> {
//         Ok(Self {
//             objects: Arc::new(RwLock::new(BTreeMap::defaul.())),
//         })
//     }
// }

#[async_trait::async_trait]
impl S3 for S3Btree {
    #[tracing::instrument(skip_all)]
    async fn get_object(
        &self,
        req: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        let GetObjectInput { key, range, .. } = req.input;

        let binding = self.objects.read().await;
        let value = binding.btree.get(&key);

        let value = match value {
            Some(value) => value,
            None => {
                return Err(s3_error!(NoSuchKey));
            }
        };
        let value = value.clone();

        // let object_path = self.get_object_path(&input.bucket, &input.key)?;

        // let mut file = fs::File::open(&object_path).await.map_err(|e| s3_error!(e, NoSuchKey))?;

        // let file_metadata = try_!(file.metadata().await);
        //let last_modified = Timestamp::from(try_!(file_metadata.modified()));
        let last_modified = Timestamp::from(SystemTime::now());
        let file_len = value.len() as u64;

        let content_length = match range {
            None => file_len,
            Some(range) => {
                let file_range = range.check(file_len)?;
                file_range.end - file_range.start
            }
        };
        let _content_length_usize = try_!(usize::try_from(content_length));
        let content_length_i64 = try_!(i64::try_from(content_length));

        // match input.range {
        //     Some(Range::Int { first, .. }) => {
        //         try_!(file.seek(io::SeekFrom::Start(first)).await);
        //     }
        //     Some(Range::Suffix { length }) => {
        //         let neg_offset = length.numeric_cast::<i64>().neg();
        //         try_!(file.seek(io::SeekFrom::End(neg_offset)).await);
        //     }
        //     None => {}
        // }

        // let body = bytes_stream(ReaderStream::with_capacity(file, 4096), content_length_usize);
        let foo = value.clone();
        let foo = Bytes::from(foo);

        let vec_stream = VecByteStream::new(vec![foo]);

        let streaming_blob = StreamingBlob::new(vec_stream);

        let object_metadata = None;

        //let md5_sum = self.get_md5_sum(&input.bucket, &input.key).await?;

        let mut md5_hash = Md5::new();
        md5_hash.update(&value);
        let md5_sum = hex(md5_hash.finalize());

        let e_tag = format!("\"{md5_sum}\"");

        //let info = self.load_internal_info(&input.bucket, &input.key).await?;
        // let checksum = match &info {
        //     Some(info) => crate::checksum::from_internal_info(info),
        //     None => default(),
        // };
        let _checksum: Option<String> = None;

        let output = GetObjectOutput {
            //  body: Some(StreamingBlob::wrap(streaming_blob)),
            body: Some(streaming_blob),
            //body: None,
            content_length: content_length_i64,
            last_modified: Some(last_modified),
            metadata: object_metadata,
            e_tag: Some(e_tag),
            checksum_crc32: None,  //checksum.checksum_crc32,
            checksum_crc32c: None, // checksum.checksum_crc32c,
            checksum_sha1: None,   // checksum.checksum_sha1,
            checksum_sha256: None, // checksum.checksum_sha256,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }

    #[tracing::instrument(skip_all)]
    async fn put_object(
        self: &S3Btree,
        req: S3Request<PutObjectInput>,
    ) -> S3Result<S3Response<PutObjectOutput>> {
        let input = req.input;
        let PutObjectInput {
            body,
            key,
            storage_class,
            ..
        } = input;

        #[cfg(fuzzing_repro)]
        {
            use fuzzing::BackendS3Instructions;
            use serde::{Deserialize, Serialize};
            let deser: BackendS3Instructions = serde_json::from_str(&bucket).unwrap();
            bucket = deser.real_bucket;

            let mut state = self.objects.write().await;
            println!("btree put fuzz {:?}", deserialized);

          
            if deser.rand_fail && state.fake_rand {
                return Err(s3_error!(InternalError));
            }
            state.fake_rand = !state.fake_rand;
        }

        if let Some(ref storage_class) = storage_class {
            let is_valid = ["STANDARD", "REDUCED_REDUNDANCY"].contains(&storage_class.as_str());
            if !is_valid {
                return Err(s3_error!(InvalidStorageClass));
            }
        }

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
        objects.btree.insert(key, vec.to_owned());

        return Ok(S3Response::new(PutObjectOutput::default()));
    }

    #[tracing::instrument(skip_all)]
    async fn delete_object(
        &self,
        req: S3Request<DeleteObjectInput>,
    ) -> S3Result<S3Response<DeleteObjectOutput>> {
        let input = req.input;

        let DeleteObjectInput { key, .. } = input;

        if self.objects.write().await.btree.remove(&key).is_none() {
            return Err(s3_error!(NoSuchKey));
        }

        return Ok(S3Response::new(DeleteObjectOutput::default()));
    }

    #[tracing::instrument(skip_all)]
    async fn head_object(
        &self,
        req: S3Request<HeadObjectInput>,
    ) -> S3Result<S3Response<HeadObjectOutput>> {
        let input = req.input;

        let key = input.key;
        let binding = self.objects.read().await;
        let value = binding.btree.get(&key);

        let value = match value {
            Some(value) => value,
            None => {
                return Err(s3_error!(NoSuchKey));
            }
        };

        let len = value.len();

        let last_modified = Timestamp::from(SystemTime::now());
        let object_metadata = None;

        // TODO: detect content type
        let content_type = mime::APPLICATION_OCTET_STREAM;

        let output = HeadObjectOutput {
            content_length: try_!(i64::try_from(len)),
            content_type: Some(content_type),
            last_modified: Some(last_modified),
            metadata: object_metadata,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }
}

// from s3s, s3s copyright stands
pub fn hex(input: impl AsRef<[u8]>) -> String {
    hex_simd::encode_to_string(input.as_ref(), hex_simd::AsciiCase::Lower)
}
