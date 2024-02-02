# s3-kit

A toolkit for building higher level S3 applications. 
The goal of this project is to eventually provide a robust distributed erasure coded S3 server that can be used locally.

## Examples

This crate provides three examples of S3 protocol servers:

- `examples/filesystem`     - A simple POC filesystem server that serves files from the `/tmp` directory.
- `examples/btree`          - A simple POC in-memory server that serves files from a `BTree`.
- `examples/replication`    - A simple POC in-memory server that does replication to two BTree S3 instances using Rust generics.

## Usage

This is a small tutorial on getting starter with any of the s3-kit examples.

The examples all use 'x' as the access key and 'x' as the secret key.

You can use the `aws` cli tool to interact with the examples. Install it from [here][1] You can setup a credentials file up, like so:

```bash
$ cat ~/.aws/credentials
```
output:
```
[default]
aws_access_key_id = x
aws_secret_access_key = x
```

You can then run one of the examples like so:
```bash
cargo run --example filesystem
```
output:
```
[ building messages ]
filesystem server is running at http://127.0.0.1:8080
```

You can then use the `aws` cli tool to interact with the server:
```bash
cat 'hello world' > hello.txt
aws  --endpoint-url http://localhost:8080 s3 cp hello.txt s3://bkt/hello
aws  --endpoint-url http://localhost:8080 s3 cp s3://bkt/hello hello2.txt
cat hello2.txt
``` 

## License

s3-kit is licensed under both the MIT license and the Apache License (Version 2.0).







[1]: https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html#getting-started-install-instructions