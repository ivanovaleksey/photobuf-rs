use futures::future::{self, Either, Loop};
use hyper::rt;
use hyper::rt::Future;
use hyper::rt::Stream;
use hyper::{self, Request};
use hyper_tls;
use quicli::prelude::*;
use tokio_timer;

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

const URL: &str = "https://cloud-api.yandex.net/v1/disk/resources/upload";

pub fn call(dir: PathBuf) -> Result<()> {
    let f = future::loop_fn(fetch(&dir), |mut chunks| {
        if let Some(chunk) = chunks.pop() {
            let t = tokio_timer::sleep(Duration::from_secs(10)).map_err(|_| ());
            let a = chunk.join(t).and_then(|_| Ok(Loop::Continue(chunks)));
            Either::A(a)
        } else {
            Either::B(future::ok(Loop::Break(())))
        }
    });

    rt::run(f);

    Ok(())
}

fn fetch(dir: &Path) -> Vec<impl Future<Item = (), Error = ()>> {
    use build_client;

    let client = build_client();
    let client = Arc::new(client);

    let entries = fs::read_dir(&dir).unwrap();
    let entries = entries.into_iter().collect::<Vec<_>>();

    let mut chunk_futures = vec![];

    for chunk in entries.chunks(5) {
        debug!("+++ Chunk +++");

        let mut futures = vec![];

        for entry in chunk {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                let name = name.into_string().unwrap();
                debug!("Handle {}", name);

                futures.push(fetch_url(client.clone(), &name));
            }
        }

        let f = future::join_all(futures).map(|_| ());
        chunk_futures.push(f);
    }

    chunk_futures
}

fn fetch_url(
    client: Arc<hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>>,
    file_name: &str,
) -> impl Future<Item = (), Error = ()> {
    let uri = format!(
        "{}?path=disk:/sandbox/{}&overwrite=false",
        URL,
        file_name.replace(' ', "%20")
    );
    debug!("URI: {}", uri);

    let request = Request::get(uri)
        .header(
            "Authorization",
            format!("OAuth {}", ::std::env::var("YANDEX_OAUTH_TOKEN").unwrap()).as_str(),
        )
        .header("Content-Type", "application/json")
        .body(hyper::Body::default())
        .unwrap();

    client.request(request).map_err(|_| ()).and_then(|res| {
        debug!("Response: {}", res.status());
        debug!("Headers: {:#?}", res.headers());

        res.into_body()
            .concat2()
            .and_then(|body| {
                io::stdout().write_all(&body).unwrap();
                Ok(())
            })
            .map_err(|_| ())
    })
}
