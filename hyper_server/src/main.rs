use bytes::Bytes;
use http_body_util::Empty;
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);

    let res = client.get("https://hyper.rs".parse()?).await?;
    println!("result: {:#?}", res);
    assert_eq!(res.status(), 200);
    Ok(())
}   