#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use futures::stream::FuturesUnordered;
use futures::StreamExt;

use derivative::Derivative;

use s3s::dto::builders::HeadObjectInputBuilder;
use s3s::dto::GetObjectInput;
use s3s::dto::GetObjectOutput;
use s3s::dto::*;
use s3s::s3_error;

use s3s::S3Result;

use s3s::S3;
use s3s::{S3Request, S3Response};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct S3Replication<T: S3 + Default> {
    #[derivative(Debug = "ignore")]
    //target: Box<dyn S3>,
    tgts: Vec<T>,
}

impl<T: S3 + Default> Default for S3Replication<T> {
    fn default() -> Self {
        Self {
            // target1: T::default(),
            // target2: T::default(),
            tgts: vec![T::default(), T::default()],
        }
    }
}

// impl<T: S3 + Default> S3Replication<T> {
//     pub fn new() -> Result<Self> {
//         let slf = Self {
//             target1: T::default(),
//             target2: T::default(),
//         };
//         Ok(slf)
//     }
// }

#[async_trait::async_trait]
impl<T: S3 + std::fmt::Debug + Default> S3 for S3Replication<T> {
    #[tracing::instrument]
    async fn get_object(
        &self,
        _req: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        Err(s3_error!(NoSuchKey))
    }

    async fn put_object(
        self: &S3Replication<T>,
        _req: S3Request<PutObjectInput>,
    ) -> S3Result<S3Response<PutObjectOutput>> {
        Err(s3_error!(NoSuchKey))
    }

    async fn delete_object(
        &self,
        _req: S3Request<DeleteObjectInput>,
    ) -> S3Result<S3Response<DeleteObjectOutput>> {
        Err(s3_error!(NoSuchKey))
    }

    #[tracing::instrument(skip_all)]
    async fn head_object(
        &self,
        _r: S3Request<HeadObjectInput>,
    ) -> S3Result<S3Response<HeadObjectOutput>> {
        //
        // let mut set = JoinSet::new();

        let HeadObjectInput { bucket, key, .. } = _r.input;

        let fo = FuturesUnordered::new();
        for x in self.tgts.iter() {
            let b = HeadObjectInputBuilder::default()
                .bucket(bucket.clone())
                .key(key.clone())
                .build()
                .unwrap();

            let f1 = x.head_object(S3Request::new(b));

            fo.push(f1);
        }

        let mut results = fo.collect::<Vec<_>>().await;

        let all_ok = results.iter().all(|x| x.is_ok());
        let all_err = results.iter().all(|x| x.is_err());

        if all_err {
            let errs = results
                .iter()
                .filter(|x| x.is_err()) // redundant
                .map(|x| match x {
                    Ok(_) => "".to_string(),
                    Err(e) => format!("{:?}", e),
                })
                .collect::<Vec<_>>();

            if all_elements_equal(&errs) {
                let o = results.remove(0);
                return match o {
                    Ok(_) => panic!(),
                    Err(e) => Err(e),
                };
            }
        } else if all_ok {
            let ok = results
                .iter()
                .filter(|x| x.is_ok()) // redundant
                .map(|x| match x {
                    Ok(kk) => (
                        kk.output.content_length,
                        //    kk.output.content_type,
                        //  kk.output.last_modified,
                        //  kk.output.metadata.clone(),// XX slow!
                    ),
                    Err(_e) => panic!(),
                })
                .collect::<Vec<_>>();

            if all_elements_equal(&ok) {
                return results.remove(0);
            }
        }
        let first_ok_index = results.iter().position(|x| x.is_ok()).unwrap();

        return results.remove(first_ok_index);

        // while let Some(result) = fo.next().await {
        //     // Handle result
        //     info!("Got result: is_ok: {}", result.is_ok());
        // }

        //XXXX
        // // TODO: detect content type
        // let content_type = mime::APPLICATION_OCTET_STREAM;

        // let output = HeadObjectOutput {
        //     content_length: try_!(i64::try_from(len)),
        //     content_type: Some(content_type),
        //     last_modified: None,
        //     metadata: None,
        //     ..Default::default()
        // };
        // Ok(S3Response::new(output))
    }
}

fn all_elements_equal<T: PartialEq>(vec: &[T]) -> bool {
    if vec.is_empty() {
        return true; // or false, depending on your definition for empty vectors
    }

    let first = &vec[0];
    vec.iter().all(|item| item == first)
}

// from s3s, s3s copyright stands
pub fn hex(input: impl AsRef<[u8]>) -> String {
    hex_simd::encode_to_string(input.as_ref(), hex_simd::AsciiCase::Lower)
}
