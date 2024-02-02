#![no_main]

use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target};
use s3_kit::fuzzing::BackendS3Instructions;
use s3_kit::vec_byte_stream::VecByteStream;
use s3_kit::{s3_btree::S3Btree, s3_replication::S3Replication};
use s3s::dto::StreamingBlob;
use s3s::dto::{DeleteObjectInput, GetObjectInput, PutObjectInput};
use s3s::S3;
use s3s::{dto::HeadObjectInput, S3Request};

use std::sync::Once;
use tokio::runtime::Runtime;

static _INIT: Once = Once::new();

#[derive(Debug, Arbitrary)]
enum Operation {
    Put { empty_body: bool },
    Get,
    Head,
    Delete,
}

#[derive(Debug, Arbitrary)]
struct Action {
    front_op: Operation,
    back_instructions: BackendS3Instructions,
}

fuzz_target!(|x: Vec<Action>| {
    //  println!("fuzz_target");

    //let x = vec![MyEnum::Put];

    //  INIT.call_once(|| foo::tracing::setup_tracing());

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        my_async_function(x).await.unwrap();
    });
});

async fn my_async_function(x: Vec<Action>) -> Result<(), ()> {
    // Process the data asynchronously

    // info!("my_async_function");

    //println!("len {:?}", x.len());

    let fs: S3Replication<S3Btree> = S3Replication::default();

    for i in x {
        match i.front_op {
            Operation::Put { empty_body } => {
                //this is how we pass data to the backend
                let bucket_js = bitcode::encode(&i.back_instructions).unwrap();
                let bucket_js = unsafe { String::from_utf8_unchecked(bucket_js) };

                //serde_json::to_string(&i.back_instructions).unwrap();

                let body = bytes::Bytes::from(b"foof".to_vec());
                let sb = StreamingBlob::new(VecByteStream::new(vec![body]));
                let body = match empty_body {
                    true => None,
                    false => Some(sb),
                };
                let r = S3Request::new(
                    PutObjectInput::builder()
                        .bucket(bucket_js)
                        .key("key".to_string())
                        .body(body)
                        .build()
                        .unwrap(),
                );

                let _a = fs.put_object(r).await;
                // println!("put ok {} not {}", a.is_ok(), a.is_err());
            }
            Operation::Get => {
                let r = S3Request::new(
                    GetObjectInput::builder()
                        .bucket("bucket".to_string())
                        .key("key".to_string())
                        .build()
                        .unwrap(),
                );

                let _a = fs.get_object(r).await;

                //println!("get ok {} not {}", a.is_ok(), a.is_err());
            }
            Operation::Head => {
                let r = S3Request::new(
                    HeadObjectInput::builder()
                        .bucket("bucket".to_string())
                        .key("key".to_string())
                        .build()
                        .unwrap(),
                );

                let _a = fs.head_object(r).await;

                //println!("head ok {} not {}", a.is_ok(), a.is_err());
            }
            Operation::Delete => {
                let r = S3Request::new(
                    DeleteObjectInput::builder()
                        .bucket("bucket".to_string())
                        .key("key".to_string())
                        .build()
                        .unwrap(),
                );

                let _a = fs.delete_object(r).await;

                //println!("del ok {} not {}", a.is_ok(), a.is_err());

                //dump_delete(a);
            }
        }
    }

    Ok(())
}

// #[::tracing::instrument]
// fn dump_delete(a: Result<S3Response<DeleteObjectOutput>, S3Error>) {
//     match a {
//         Ok(_) => println!("delete ok"),
//         Err(e) => println!("delete err {:?}", e),
//     }
// }
