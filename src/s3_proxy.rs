#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

// S3DEC
// Dec stands for distributed erasure coding.mod s3_btree;



use hyper::service::Service;

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

type Z = Box<
    dyn Service<
            S3Request<GetObjectInput>,
            Response = S3Result<S3Response<GetObjectOutput>>,
            Error = Error,
            Future = dyn futures::Future<Output = Result<S3Result<S3Response<GetObjectOutput>>>>
                         + Send,
        > + Send,
>;

#[macro_use]
use derivative::Derivative;
//#[derive(Debug)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct S3ServiceProxy {
    //x: Arc<RwLock<Z>>,
    #[derivative(Debug = "ignore")]
    x: MakeService<SharedS3Service>,
}

impl S3ServiceProxy {
    pub fn new() -> Result<Self> {
        let btree = S3Btree::new()?;
        let service = {
            let b = S3ServiceBuilder::new(btree);
            b.build()
        };
        let sss = service.into_shared().into_make_service();

        Ok(Self {
            //x: Arc::new(RwLock::new(sss)),
            x: sss,
        })
    }
}

#[async_trait::async_trait]
impl S3 for S3ServiceProxy {
    #[tracing::instrument]
    async fn get_object(
        &self,
        _req: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        //
        //self.target.get_object(req).await
        //
        return Err(s3_error!(NoSuchKey));
    }

    async fn put_object(
        self: &S3ServiceProxy,
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
        //
        //self.target.head_object(req).await
        //
        return Err(s3_error!(NoSuchKey));
    }
}
