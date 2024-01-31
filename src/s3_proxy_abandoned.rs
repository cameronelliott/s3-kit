#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

// S3DEC
// Dec stands for distributed erasure coding.mod s3_btree;

use hyper::service::Service;
use hyper::Request;
use s3s::dto::GetObjectInput;
use s3s::dto::GetObjectOutput;
use s3s::dto::*;

use s3s::S3Result;

use s3s::s3_error;
use s3s::service::MakeService;
use s3s::service::S3ServiceBuilder;
use s3s::service::SharedS3Service;
use s3s::S3;
use s3s::{S3Request, S3Response};

use crate::error::Result;

use crate::s3_btree::S3Btree;

/*
this is an interim test before writing the distributed erasure coding
*/

// type XXX = dyn Service<
//     S3Request<GetObjectInput>,
//     Response = S3Result<S3Response<GetObjectOutput>>,
//     Error = Error,
//     Future = dyn Future<Output = Result<S3Result<S3Response<GetObjectOutput>>>> + Send,
// >;

// type Z = Box<
//     dyn Service<
//             S3Request<GetObjectInput>,
//             Response = S3Result<S3Response<GetObjectOutput>>,
//             Error = Error,
//             Future = dyn futures::Future<Output = Result<S3Result<S3Response<GetObjectOutput>>>>
//                          + Send,
//         > + Send,
// >;

use derivative::Derivative;

//#[derive(Debug)]

// pub struct Proxy {
//     //x: Arc<RwLock<Z>>,
//     #[derivative(Debug = "ignore")]
//     target_service: MakeService<SharedS3Service>,
// }

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Proxy {
    #[derivative(Debug = "ignore")]
    client: aws_sdk_s3::Client,
}

impl From<aws_sdk_s3::Client> for Proxy {
    fn from(value: aws_sdk_s3::Client) -> Self {
        Self { client: value}
    }
}

impl Proxy {
    pub fn new(client:aws_sdk_s3::Client) -> Result<Self> {
        let btree = S3Btree::default();
        let service = {
            let b = S3ServiceBuilder::new(btree);
            b.build()
        };
        let service = service.into_shared().into_make_service();

        Ok(Self {
            //x: Arc::new(RwLock::new(sss)),
            target_service: service,
            client: client,
        })
    }
}

#[async_trait::async_trait]
impl S3 for Proxy {
    #[tracing::instrument]
    async fn get_object(
        &self,
        req: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        return Err(s3_error!(NoSuchKey));
    }

    async fn put_object(
        self: &Proxy,
        _req: S3Request<PutObjectInput>,
    ) -> S3Result<S3Response<PutObjectOutput>> {
        //
        // self.target.put_object(req).await
        //
        return Err(s3_error!(NoSuchKey));
    }

    async fn delete_object(
        &self,
        _req: S3Request<DeleteObjectInput>,
    ) -> S3Result<S3Response<DeleteObjectOutput>> {
        //
        // self.target.delete_object(req).await
        //
        return Err(s3_error!(NoSuchKey));
    }

    #[tracing::instrument]
    async fn head_object(
        &self,
        _req: S3Request<HeadObjectInput>,
    ) -> S3Result<S3Response<HeadObjectOutput>> {
        let a = self.target_service.call("nada").await;
        let a = a.unwrap();

        let hyper_request = Request::new("fake body".into());

        let a = a.call(hyper_request).await;

        let a = a.unwrap();

        let x: S3Result<S3Response<HeadObjectOutput>> = a;

        return x;
    }
}
