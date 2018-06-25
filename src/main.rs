extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate quicli;
extern crate tokio_timer;

use hyper::{client, Client};
use hyper_tls::HttpsConnector;
use quicli::prelude::*;

mod cli;

main!(|args: cli::Cli, log_level: verbosity| {
    use cli::Command::*;

    let res = match args.cmd {
        Pack {
            source,
            destination,
        } => cli::pack::call(source, destination),
        Ping => cli::ping::call(),
        Sync { directory } => cli::sync::call(directory),
    };

    res.expect("Command failed")
});

pub fn build_client() -> Client<HttpsConnector<client::HttpConnector>> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    Client::builder().build::<_, hyper::Body>(https)
}
