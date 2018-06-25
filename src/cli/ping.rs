use hyper::rt::{self, Future, Stream};
use hyper::{self, Request};
use quicli::prelude::*;

use std::io::{self, Write};

pub fn call() -> Result<()> {
    rt::run(fetch().map_err(|_| ()));
    Ok(())
}

fn fetch() -> impl Future<Item = (), Error = Error> {
    use build_client;
    let client = build_client();

    let request = Request::get("https://cloud-api.yandex.net/v1/disk/")
        .header(
            "Authorization",
            format!("OAuth {}", ::std::env::var("YANDEX_OAUTH_TOKEN").unwrap()).as_str(),
        )
        .header("Content-Type", "application/json")
        .body(hyper::Body::default())
        .unwrap();

    client.request(request).from_err().and_then(|res| {
        debug!("Response: {}", res.status());
        debug!("Headers: {:#?}", res.headers());

        res.into_body()
            .concat2()
            .and_then(|body| {
                io::stdout().write_all(&body).unwrap();
                Ok(())
            })
            .from_err()
    })
}
