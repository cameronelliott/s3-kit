#![no_main]

use foo::{s3_btree::S3Btree, s3_replication::S3Replication};
use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target};
use s3s::dto::{DeleteObjectInput, GetObjectInput, PutObjectInput};
use s3s::S3;
use s3s::{dto::HeadObjectInput, S3Request};
use tokio::runtime::Runtime;

#[derive(Debug, Arbitrary)]
enum MyEnum {
    Put,
    Get,
    Head,
    Delete,
}

// this fuzzer is not for correctness, but
// just coverage!!!

async fn _my_async_function_example(data: &[u8]) -> Result<(), ()> {
    // Process the data asynchronously
    Ok(())
}

fuzz_target!(|x: Vec<MyEnum>| {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        my_async_function(x).await.unwrap();
    });
});

async fn my_async_function(x: Vec<MyEnum>) -> Result<(), ()> {
    // Process the data asynchronously

    println!("len {:?}", x);

    let fs: S3Replication<S3Btree> = S3Replication::default();

    for i in x {
        match i {
            MyEnum::Put => {
                let r = S3Request::new(
                    PutObjectInput::builder()
                        .bucket("bucket".to_string())
                        .key("key".to_string())
                        .build()
                        .unwrap(),
                );

                _ = fs.put_object(r).await;
            }
            MyEnum::Get => {
                let r = S3Request::new(
                    GetObjectInput::builder()
                        .bucket("bucket".to_string())
                        .key("key".to_string())
                        .build()
                        .unwrap(),
                );

                _ = fs.get_object(r).await;
            }
            MyEnum::Head => {
                let r = S3Request::new(
                    HeadObjectInput::builder()
                        .bucket("bucket".to_string())
                        .key("key".to_string())
                        .build()
                        .unwrap(),
                );

                _ = fs.head_object(r).await;
            }
            MyEnum::Delete => {
                let r = S3Request::new(
                    DeleteObjectInput::builder()
                        .bucket("bucket".to_string())
                        .key("key".to_string())
                        .build()
                        .unwrap(),
                );

                _ = fs.delete_object(r).await;
            }
        }
    }

    Ok(())
}
