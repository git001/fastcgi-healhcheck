use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    routing::get,
    Router,
};
use kvarn_fastcgi_client::{Client, Params, Request as FCGIRequest, Response as FCGIResponse};
use std::{env, net::ToSocketAddrs};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

const DEFAULT_SERVER_ADDR_PORT: &str = "127.0.0.1:8080";
const DEFAULT_DEST_PORT: u16 = 9000;
const DEFAULT_DEST_ADDR: &str = "127.0.0.1";
const DEFAULT_PING_PATH: &str = "/fpm-ping";
const DEFAULT_PING_RESPONSE: &str = "pong";

async fn hello_world() -> &'static str {
    "Hello world!\n"
}

//&'static str
async fn check_fcgi_endpoint(_headers: HeaderMap, _req: Request) -> StatusCode {
    // println!("Request Headers {:?}", headers);
    // println!("Request {:?}", req);
    // println!("Request URI :{:?}:", req.uri());

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

    // println!("Dest to addr {}", destination_addr);
    // println!("Dest to port {}", destination_port);
    // println!("ping_path {}", ping_path);
    // println!("ping_response {}", ping_path);

    let php_addr = (destination_addr.clone(), destination_port)
        .to_socket_addrs()
        .expect("Unable to resolve the IP address")
        .next()
        .expect("DNS resolution returned no IP addresses");

    //let dest_connection = TcpStream::connect(&php_addr).await.expect("msg");
    let dest_connection = match TcpStream::connect(&php_addr).await {
        Ok(dest_connection) => dest_connection,
        // TODO: Handle this better
        Err(e) => panic!("{}", e),
    };

    //println!("Dest to conn {:?}", dest_connection);
    let client = Client::new(dest_connection);

    // Fastcgi params, please reference to nginx-php-fpm config.
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
        .execute_once(FCGIRequest::new(params, &mut io::empty()))
        .await
        .unwrap();
    //println!("output {:#?}", output.borrow());

    // stdout "X-Powered-By: PHP/8.1.2-1ubuntu2.14\r\nContent-type: text/plain;charset=UTF-8\r\nExpires: Thu, 01 Jan 1970 00:00:00 GMT\r\nCache-Control: no-cache, no-store, must-revalidate, max-age=0\r\n\r\npong"
    let stdout = String::from_utf8(output.stdout.unwrap()).unwrap();

    //println!("output {:?}", output.clone());
    //println!("stdout {:?}", stdout);
    if stdout.contains(&ping_response) {
        return StatusCode::OK;
    } else {
        println!("ping_path {:?}", ping_path);
        println!("stdout {:?}", stdout);
        return StatusCode::NOT_FOUND;
    }
}

#[tokio::main]
async fn main() {
    let server_addr = env::var("SERVER")
        .ok()
        .unwrap_or(DEFAULT_SERVER_ADDR_PORT.to_string());

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(hello_world))
        .route("/healthz", get(hello_world))
        .route("/fcgi-ping", get(check_fcgi_endpoint));

    println!("Listening on {}", server_addr);

    let tcp = TcpListener::bind(&server_addr).await.unwrap();

    axum::serve(tcp, router).await.unwrap();
}
