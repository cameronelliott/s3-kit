#![no_main]

use foo::vec_byte_stream::VecByteStream;
use foo::{s3_btree::S3Btree, s3_replication::S3Replication};
use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target};
use s3s::dto::StreamingBlob;
use s3s::dto::{DeleteObjectInput, GetObjectInput, PutObjectInput};
use s3s::S3;
use s3s::{dto::HeadObjectInput, S3Request};
use std::sync::Once;
use tokio::runtime::Runtime;
use tracing::info;

static _INIT: Once = Once::new();

#[derive(Debug, Arbitrary)]
enum MyEnum {
    Put,
    Get,
    Head,
    Delete,
}

// this fuzzer is not for correctness, but
// just coverage!!!

// async fn _my_async_function_example(_data: &[u8]) -> Result<(), ()> {
//     // Process the data asynchronously
//     _ = _data;
//     Ok(())
// }

fuzz_target!(|x: Vec<MyEnum>| {
    //  println!("fuzz_target");

    //let x = vec![MyEnum::Put];

    //  INIT.call_once(|| foo::tracing::setup_tracing());

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        my_async_function(x).await.unwrap();
    });
});

async fn my_async_function(x: Vec<MyEnum>) -> Result<(), ()> {
    // Process the data asynchronously

   // info!("my_async_function");

    //println!("len {:?}", x.len());

    let fs: S3Replication<S3Btree> = S3Replication::default();

    for i in x {
        match i {
            MyEnum::Put => {
                let foo = bytes::Bytes::from(b"".to_vec());
                let sb = StreamingBlob::new(VecByteStream::new(vec![foo]));
                let r = S3Request::new(
                    PutObjectInput::builder()
                        .bucket("bucket".to_string())
                        .key("key".to_string())
                        .body(Some(sb))
                        .build()
                        .unwrap(),
                );

                let _a = fs.put_object(r).await;
                // println!("put ok {} not {}", a.is_ok(), a.is_err());
            }
            MyEnum::Get => {
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
            MyEnum::Head => {
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
            MyEnum::Delete => {
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
