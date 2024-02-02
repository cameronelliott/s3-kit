#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
// https://github.com/rust-lang/rust/issues/63063
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

use s3s::auth::{self};
use s3s::service::S3ServiceBuilder;

use s3s_fs::FileSystem;

use std::env::args;
use std::net::TcpListener;
use std::path::Path;

use hyper::server::Server;

use s3_kit::error::*;

#[tokio::main]
async fn main() -> Result {
    env_logger::init();

    let flags = xflags::parse_or_exit! {
        ///listener port
        optional -p, --port port: u16
        /// bind address
        optional -b,--bindaddr bindaddr: String
    };

    let port = flags.port.unwrap_or(8080);
    let bindaddr = flags.bindaddr.unwrap_or("127.0.0.1".to_string());

    let rootpath = Path::new("/tmp");
    let fs = FileSystem::new(rootpath).expect("unable to access /tmp for filesystem");

    // Setup S3 service
    let service = {
        let mut b = S3ServiceBuilder::new(fs);

        b.set_auth(auth::SimpleAuth::from_single(
            "x".to_string(),
            "x".to_string(),
        ));

        b.build()
    };

    let listener = TcpListener::bind((bindaddr, port))?;
    let local_addr = listener.local_addr()?;
    let server = Server::from_tcp(listener)?.serve(service.into_shared().into_make_service());
    let my_path = &args().next().unwrap();
    let basename = Path::new(my_path).file_name().unwrap().to_str().unwrap();
    println!("{} server is running at http://{local_addr}", basename);
    server.with_graceful_shutdown(shutdown_signal()).await?;
    println!("server is stopped");
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
