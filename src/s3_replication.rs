use bytes::Bytes;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

use s3s::dto::builders::DeleteObjectInputBuilder;
use s3s::dto::builders::GetObjectInputBuilder;

use s3s::dto::builders::PutObjectInputBuilder;
use s3s::dto::GetObjectInput;
use s3s::dto::GetObjectOutput;
use s3s::dto::*;
use s3s::s3_error;

use s3s::S3Result;

use s3s::S3;
use s3s::{S3Request, S3Response};

use crate::fix_await_coverage::FixAwaitCoverage;
use crate::utils::copy_bytes;
use crate::vec_byte_stream::VecByteStream;

// #[derive(Derivative)]
// #[derivative(Debug)]
pub struct S3Replication<T: S3 + Default> {
    //#[derivative(Debug = "ignore")]
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
    #[tracing::instrument(skip_all)]
    async fn get_object(
        &self,
        r: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        //
        // let mut set = JoinSet::new();
        let GetObjectInput { bucket, key, .. } = r.input;

        let fo = FuturesUnordered::new();
        for x in self.tgts.iter() {
            let b = GetObjectInputBuilder::default()
                .bucket(bucket.clone())
                .key(key.clone())
                .build()
                .unwrap();

            let f1 = x.get_object(S3Request::new(b));

            fo.push(f1);
        }

        let mut results = fo.collect::<Vec<_>>().fix_cov().await;

        let all_ok = results.iter().all(|x| x.is_ok());
        let all_err = results.iter().all(|x| x.is_err());

        if all_err {
            // let _errs = results
            //     .iter()
            //     .filter(|x| x.is_err()) // redundant
            //     // .map(|x| match x {
            //     //     Ok(_) => "".to_string(),
            //     //     Err(e) => format!("{:?}", e),
            //     // })
            //     .map(|x| match x {
            //         Ok(_) => todo!(),
            //         Err(e) => format!("{:?}", e),
            //     })
            //     .collect::<Vec<_>>();

            // if all_elements_equal(&errs) {
            //     let o = results.remove(0);
            //     return match o {
            //         Ok(_) => panic!(),
            //         Err(e) => Err(e),
            //     };
            // }
            return results.remove(0);
        } else if all_ok {
            // let _ok = results
            //     .iter()
            //     .filter(|x| x.is_ok()) // redundant
            //     .map(|x| match x {
            //         Ok(kk) => (
            //             kk.output.content_length,
            //             //    kk.output.content_type,
            //             //  kk.output.last_modified,
            //             //  kk.output.metadata.clone(),// XX slow!
            //         ),
            //         Err(_e) => panic!(),
            //     })
            //     .collect::<Vec<_>>();

            // if all_elements_equal(&ok) {
            //     return results.remove(0);
            // }
            return results.remove(0);
        }
        //else if !all_ok && !all_err {
        //mixed results

        let first_ok_index = results.iter().position(|x| x.is_ok()).unwrap();

        return results.remove(first_ok_index);
    }

    #[tracing::instrument(skip_all)]
    async fn put_object(
        self: &S3Replication<T>,
        req: S3Request<PutObjectInput>,
    ) -> S3Result<S3Response<PutObjectOutput>> {
        //
        let PutObjectInput {
            key, bucket, body, ..
        } = req.input;

        // if there is no body, return an error
        if body.is_none() {
            return Err(s3_error!(InvalidRequest));
        }
        let stream = body.unwrap();

        let mut result = Vec::new();
        let _num_copy = copy_bytes(stream, &mut result).fix_cov().await; //do not use ?, for code coverage reasons

        let bytes = Bytes::from(result);

        let fo = FuturesUnordered::new();
        for x in self.tgts.iter() {
            let streaming_blob = StreamingBlob::new(VecByteStream::new(vec![bytes.clone()]));
            let b = PutObjectInputBuilder::default()
                .bucket(bucket.clone())
                .key(key.clone())
                .body(Some(streaming_blob.into()))
                .build()
                .unwrap();

            let f1 = x.put_object(S3Request::new(b));

            fo.push(f1);
        }

        let mut results = fo.collect::<Vec<_>>().fix_cov().await;

        let all_ok = results.iter().all(|x| x.is_ok());
        let all_err = results.iter().all(|x| x.is_err());

        let num_ok = results.iter().filter(|x| x.is_ok()).count();
        let num_err = results.iter().filter(|x| x.is_err()).count();
        println!(
            "nresults {} numok {} numerr {} all_ok {} all_err {}",
            results.len(),
            num_ok,
            num_err,
            all_ok,
            all_err
        );

        if all_err {
            // let _errs = results
            //     .iter()
            //     .filter(|x| x.is_err()) // redundant
            //     .map(|x| match x {
            //         Ok(_) => "".to_string(),
            //         Err(e) => format!("{:?}", e),
            //     })
            //     .collect::<Vec<_>>();

            // if all_elements_equal(&errs) {
            //     let o = results.remove(0);
            //     return match o {
            //         Ok(_) => panic!(),
            //         Err(e) => Err(e),
            //     };
            // }

            return results.remove(0);
        } else if all_ok {
            return results.remove(0);
        }
        // if !all_ok && !all_err {
        //mixed results

        let ok_ix = results.iter().position(|x| x.is_ok()).unwrap();

        return results.remove(ok_ix);
    }

    #[tracing::instrument(skip_all)]
    async fn delete_object(
        &self,
        req: S3Request<DeleteObjectInput>,
    ) -> S3Result<S3Response<DeleteObjectOutput>> {
        //
        let DeleteObjectInput { bucket, key, .. } = req.input;

        let fo = FuturesUnordered::new();
        for x in self.tgts.iter() {
            let b = DeleteObjectInputBuilder::default()
                .bucket(bucket.clone())
                .key(key.clone())
                .build()
                .unwrap();

            let f1 = x.delete_object(S3Request::new(b));

            fo.push(f1);
        }

        let mut results = fo.collect::<Vec<_>>().fix_cov().await;

        let all_ok = results.iter().all(|x| x.is_ok());
        let all_err = results.iter().all(|x| x.is_err());

        if all_err {
            // let _errs = results
            //     .iter()
            //     .filter(|x| x.is_err()) // redundant
            //     .map(|x| match x {
            //         Ok(_) => "".to_string(),
            //         Err(e) => format!("{:?}", e),
            //     })
            //     .collect::<Vec<_>>();

            // if all_elements_equal(&errs) {
            //     let o = results.remove(0);
            //     return match o {
            //         Ok(_) => panic!(),
            //         Err(e) => Err(e),
            //     };
            // }
            return results.remove(0);
        } else if all_ok {
            // let _ok = results
            //     .iter()
            //     .filter(|x| x.is_ok()) // redundant
            //     .map(|x| match x {
            //         Ok(kk) => (
            //             kk.output.delete_marker,
            //             //    kk.output.content_type,
            //             //  kk.output.last_modified,
            //             //  kk.output.metadata.clone(),// XX slow!
            //         ),
            //         Err(_e) => panic!(),
            //     })
            //     .collect::<Vec<_>>();

            // if all_elements_equal(&ok) {
            //     return results.remove(0);
            // }
            return results.remove(0);
        }
        //else if !all_ok && !all_err {
        //mixed results

        let first_ok_index = results.iter().position(|x| x.is_ok()).unwrap();

        return results.remove(first_ok_index);
    }

    #[tracing::instrument(skip_all)]
    async fn head_object(
        &self,
        req: S3Request<HeadObjectInput>,
    ) -> S3Result<S3Response<HeadObjectOutput>> {
        //

        let HeadObjectInput { bucket, key, .. } = req.input;

        let goi = GetObjectInputBuilder::default()
            .bucket(bucket.clone())
            .key(key.clone())
            .build()
            .unwrap();

        let goo = self.get_object(S3Request::new(goi)).fix_cov().await?;

        let GetObjectOutput {
            content_length,
            content_type,
            last_modified,
            metadata,
            ..
        } = goo.output;

        let output = HeadObjectOutput {
            content_length,
            content_type,
            last_modified,
            metadata,
            ..Default::default()
        };

        Ok(S3Response::new(output))
    }
}
