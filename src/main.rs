use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    routing::get,
    Router,
};
use chrono::Utc;
use env_logger::Builder;
use kvarn_fastcgi_client::{Client, Params, Request as FCGIRequest, Response as FCGIResponse};
use std::{env, io::Write, net::ToSocketAddrs};
use tokio::net::{TcpListener, TcpStream};

const DEFAULT_SERVER_ADDR_PORT: &str = "127.0.0.1:8080";
const DEFAULT_DEST_PORT: u16 = 9000;
const DEFAULT_DEST_ADDR: &str = "127.0.0.1";
const DEFAULT_PING_PATH: &str = "/fpm-ping";
const DEFAULT_PING_RESPONSE: &str = "pong";

async fn hello_world() -> &'static str {
    "Hello world!\n"
}

async fn check_fcgi_endpoint(headers: HeaderMap, req: Request) -> StatusCode {

    log::debug!("Request Headers {:?}", headers);
    log::debug!("Request {:?}", req);
    log::debug!("Request URI :{:?}:", req.uri());

    let destination_addr = env::var("DESTINATION")
        .ok()
        .unwrap_or(DEFAULT_DEST_ADDR.to_string());

    let destination_port = env::var("DESTINATION_PORT")
        .ok()
        .and_then(|destination_port| destination_port.parse::<u16>().ok())
        .unwrap_or(DEFAULT_DEST_PORT);

    let ping_path = env::var("PING_PATH")
        .ok()
        .unwrap_or(DEFAULT_PING_PATH.to_string());

    let ping_response = env::var("PING_RESPONSE")
        .ok()
        .unwrap_or(DEFAULT_PING_RESPONSE.to_string());

    log::debug!("destination_addr :{}:", destination_addr);
    log::debug!("destination_port :{}:", destination_port);
    log::debug!("ping_path :{}:", ping_path);
    log::debug!("ping_response :{}:", ping_response);

    let php_addr = (destination_addr.clone(), destination_port)
        .to_socket_addrs()
        .expect("Unable to resolve the IP address")
        .next()
        .expect("DNS resolution returned no IP addresses");

    //let dest_connection = TcpStream::connect(&php_addr).await.expect("msg");
    let dest_connection = match TcpStream::connect(&php_addr).await {
        Ok(dest_connection) => dest_connection,
        // TODO: Handle this better
        Err(e) => {
            log::error!("Stream error :{}:", e);
            panic!("{}", e)
        }
    };

    log::debug!("dest_connection :{:?}:", dest_connection);
    let client = Client::new(dest_connection);

    // Fastcgi params, please reference to php-fpm config.
    let params = Params::default()
        .request_method("GET")
        .script_name(ping_path.clone())
        .script_filename(ping_path.clone())
        .request_uri(ping_path.clone())
        .document_uri(ping_path.clone())
        .server_addr(destination_addr.clone())
        .server_port(destination_port);

    // Fetch fastcgi server(php-fpm) response.
    let output: FCGIResponse = client
        .execute_once(FCGIRequest::new(params, &mut tokio::io::empty()))
        .await
        .unwrap();

    // stdout "X-Powered-By: PHP/8.1.2-1ubuntu2.14\r\nContent-type: text/plain;charset=UTF-8\r\nExpires: Thu, 01 Jan 1970 00:00:00 GMT\r\nCache-Control: no-cache, no-store, must-revalidate, max-age=0\r\n\r\npong"
    let stdout = String::from_utf8(output.stdout.unwrap()).unwrap();

    if stdout.contains(&ping_response) {
        return StatusCode::OK;
    } else {
        log::error!("ping_path :{}: stdout :{}:", ping_path, stdout);
        return StatusCode::NOT_FOUND;
    }
}

#[tokio::main]
async fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} {:?}::{:?} : {}",
                record.level(),
                Utc::now().format("%Y-%m-%d %H:%M:%S %Z"),
                record.file(),
                record.line(),
                record.args()
            )
        })
        .parse_default_env()
        .init();

    let server_addr = env::var("SERVER")
        .ok()
        .unwrap_or(DEFAULT_SERVER_ADDR_PORT.to_string());

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(hello_world))
        .route("/healthz", get(hello_world))
        .route("/fcgi-ping", get(check_fcgi_endpoint));

    log::info!("Listening on {}", server_addr);

    let tcp = TcpListener::bind(&server_addr).await.unwrap();

    axum::serve(tcp, router).await.unwrap();
}
