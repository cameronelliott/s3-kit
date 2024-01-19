#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
// https://github.com/rust-lang/rust/issues/63063
#![feature(type_alias_impl_trait)]

use s3s::auth::SimpleAuth;
use s3s::service::S3ServiceBuilder;

use std::net::TcpListener;
use std::path::PathBuf;

use clap::Parser;
use hyper::server::Server;
use tracing::info;

#[macro_use]
mod error;
pub use error::Error;
pub use error::Result;

mod checksum;
mod s3_btree;
mod s3_dec;
mod s3_not_impl;
mod s3_proxy;
mod utils;
mod vec_byte_stream;
use s3_btree::S3Btree;

#[derive(Debug, Parser)]
#[command(version)]
struct Opt {
    /// Host name to listen on.
    #[arg(long, default_value = "localhost")]
    host: String,

    /// Port number to listen on.
    #[arg(long, default_value = "8014")] // The original design was finished on 2020-08-14.
    port: u16,

    /// Access key used for authentication.
    #[arg(long, requires("secret-key"))]
    access_key: Option<String>,

    /// Secret key used for authentication.
    #[arg(long, requires("access-key"))]
    secret_key: Option<String>,

    /// Domain name used for virtual-hosted-style requests.
    #[arg(long)]
    domain_name: Option<String>,

    /// Root directory of stored data.
    root: PathBuf,
}

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    let env_filter = EnvFilter::from_default_env();
    // let enable_color = std::io::stdout().is_terminal(); // TODO
    let enable_color = false;

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(env_filter)
        .with_ansi(enable_color)
        .init();
}

fn check_cli_args(opt: &Opt) -> Result<(), String> {
    if let Some(ref s) = opt.domain_name {
        if s.contains('/') {
            return Err(format!(
                "expected domain name, found URL-like string: {s:?}"
            ));
        }
    }
    Ok(())
}

fn main() -> Result {
    //print arguments
    println!("args: {:?}", std::env::args().collect::<Vec<String>>());

    let opt = Opt {
        host: "localhost".to_string(),
        port: 8014,
        access_key: "x".to_string().into(),
        secret_key: "x".to_string().into(),
        domain_name: None,
        root: PathBuf::from("/tmp"),
    };
    //println!("{:?}", opt);
    check_cli_args(&opt).map_err(error::Error::from_string)?;

    setup_tracing();

    run(opt)
}

#[tokio::main]
async fn run(opt: Opt) -> Result {
    // Setup S3 provider
    //let fs = S3Btree::new()?;
    let fs = s3_proxy::S3ServiceProxy::new()?;

    // Setup S3 service
    let service = {
        let mut b = S3ServiceBuilder::new(fs);

        // Enable authentication
        if let (Some(ak), Some(sk)) = (opt.access_key, opt.secret_key) {
            b.set_auth(SimpleAuth::from_single(ak, sk));
        }

        // Enable parsing virtual-hosted-style requests
        if let Some(domain_name) = opt.domain_name {
            b.set_base_domain(domain_name);
        }

        b.build()
    };

    // Run server
    let listener = TcpListener::bind((opt.host.as_str(), opt.port))?;
    let local_addr = listener.local_addr()?;

    let server = Server::from_tcp(listener)?.serve(service.into_shared().into_make_service());

    info!("server is running at http://{local_addr}");
    server.with_graceful_shutdown(shutdown_signal()).await?;

    info!("server is stopped");
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
