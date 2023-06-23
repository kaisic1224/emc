use http_body_util::BodyExt;
use http_body_util::Empty;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::{Request, Response};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt as _;
use tokio::net::{TcpListener, TcpStream};

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

pub async fn connect() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8664));

    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = hyper::server::conn::http1::Builder::new()
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection {:?}", err);
            }
        });
    }
}

// I just realized i have a client access token which means i dont need to do this
// life is at an all low right neow
pub async fn get_auth_token() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().expect("no env file");

    let mut base_url = "https://api.genius.com/oauth/authorize?".to_string();
    let req_params = vec![
        "client_id=".to_owned() + &env::var("CLIENT_ID").unwrap(),
        "redirect_uri=".to_owned() + "http://localhost:8664",
        "scope=me".to_string(),
        "response_type=code".to_string(),
        "state=monkeyoohoohAhAHH1".to_string(),
    ];
    base_url += &req_params.join("&");
    println!("{}", base_url);

    let url = base_url.parse::<hyper::Uri>()?;

    // get host and port
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);

    // open tcp connection to remote host @ port
    let stream = TcpStream::connect(address).await?;

    // perform handshake to verify remote is ready to receive requests
    let (mut sender, conn) = hyper::client::conn::http1::handshake(stream).await?;

    // spawn task to poll connection
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("connection failed {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let mut res = sender.send_request(req).await?;

    println!("Res status: {}", res.status());

    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            tokio::io::stdout().write_all(&chunk).await?;
        }
    }
    Ok(())
}
