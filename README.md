# Introduction

Due to the fact that in the container world is a health check quite essential and I haven’t found any simple fastcgi (fcgi) client with make such check's have I decided to create one in rust.

At the current time is the HTTP Protocol widely supported for health checks but not the fastcgi protocol.

The `fastcgi-healthcheck` tool exists for only one purpose, it calls a fcgi server with a specific URL and expect a specific answer.
Based on that answer will the tool return a 200 or a 404 HTTP code to the requester.

# Environment Variables

|Variable         | Description      | Default value | 
|--------------|---------------------|-------------|
|SERVER           | The Server and Port combination on which the tool should listen | 127.0.0.1:8080 |
|DESTINATION      | The FCGI destination address, this can be a name or ip | 127.0.0.1 |
|DESTINATION_PORT | The FCGI destination port | 9000 |
|PING_PATH        | The Ping URL on the FCGI Server | /fpm-ping |
|PING_RESPONSE    | The Ping response from the FCGI Server | pong |

I use for the FCGI call this library https://github.com/Icelk/fastcgi-client-rs

# Own health endpoints

This tool have it's own health endpoints `/health` and `/healthz`.

# Disclaimer

This is one of my first rust programm I'm pretty sure that this tool could be simplified :smiley:  
I'm open for suggestions.

# example debug run

## Scenario: PHP-FPM is not running

### health check tool running
```shell
# shell 1
RUST_LOG=debug PING_PATH=/lala SERVER=127.0.0.1:8080 DESTINATION=127.0.0.1 DESTINATION_PORT=9000 cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
Running `target/debug/fastcgi-healhcheck`
INFO 2024-04-09 11:03:53 UTC: Listening on 127.0.0.1:8080
DEBUG 2024-04-09 11:03:56 UTC: Request Headers {"host": "127.0.0.1:8080", "user-agent": "curl/7.81.0", "accept": "*/*"}
DEBUG 2024-04-09 11:03:56 UTC: Request Request { method: GET, uri: /fcgi-ping, version: HTTP/1.1, headers: {"host": "127.0.0.1:8080", "user-agent": "curl/7.81.0", "accept": "*/*"}, body: Body(UnsyncBoxBody) }
DEBUG 2024-04-09 11:03:56 UTC: Request URI :/fcgi-ping:
DEBUG 2024-04-09 11:03:56 UTC: destination_addr :127.0.0.1:
DEBUG 2024-04-09 11:03:56 UTC: destination_port :9000:
DEBUG 2024-04-09 11:03:56 UTC: ping_path :/lala:
DEBUG 2024-04-09 11:03:56 UTC: ping_response :pong:
ERROR 2024-04-09 11:03:56 UTC: Stream error :Connection refused (os error 111):
thread 'tokio-runtime-worker' panicked at src/main.rs:63:13:
Connection refused (os error 111)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### curl call

```shell
# shell 2
curl -v http://127.0.0.1:8080/fcgi-ping*   Trying 127.0.0.1:8080...
* Connected to 127.0.0.1 (127.0.0.1) port 8080 (#0)
> GET /fcgi-ping HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/7.81.0
> Accept: */*
>
* Empty reply from server
* Closing connection 0
curl: (52) Empty reply from server
```

## Scenario: PHP-FPM is running

### health check tool running
```shell
# shell 1
RUST_LOG=debug SERVER=127.0.0.1:8080 DESTINATION=127.0.0.1 DESTINATION_PORT=9000 cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.02s
Running `target/debug/fastcgi-healhcheck`
INFO 2024-04-09 11:07:44 UTC: Listening on 127.0.0.1:8080
DEBUG 2024-04-09 11:07:45 UTC: Request Headers {"host": "127.0.0.1:8080", "user-agent": "curl/7.81.0", "accept": "*/*"}
DEBUG 2024-04-09 11:07:45 UTC: Request Request { method: GET, uri: /fcgi-ping, version: HTTP/1.1, headers: {"host": "127.0.0.1:8080", "user-agent": "curl/7.81.0", "accept": "*/*"}, body: Body(UnsyncBoxBody) }
DEBUG 2024-04-09 11:07:45 UTC: Request URI :/fcgi-ping:
DEBUG 2024-04-09 11:07:45 UTC: destination_addr :127.0.0.1:
DEBUG 2024-04-09 11:07:45 UTC: destination_port :9000:
DEBUG 2024-04-09 11:07:45 UTC: ping_path :/fpm-ping:
DEBUG 2024-04-09 11:07:45 UTC: ping_response :pong:
DEBUG 2024-04-09 11:07:45 UTC: dest_connection :PollEvented { io: Some(TcpStream { addr: 127.0.0.1:60274, peer: 127.0.0.1:9000, fd: 8 }) }:
DEBUG 2024-04-09 11:07:45 UTC: Start handle request id=1
DEBUG 2024-04-09 11:07:45 UTC: Send to stream. id=1 begin_request_rec="BeginRequestRec {header: Header { version: 1, type: BeginRequest, request_id: 1, content_length: 8, padding_length: 0, reserved: 0 }, begin_request: BeginRequest { role: Responder, flags: 0, reserved: [0, 0, 0, 0, 0] }}"
DEBUG 2024-04-09 11:07:45 UTC: Params will be sent. id=1 param_pairs=ParamPairs([ParamPair { name_length: Short(15), value_length: Short(17), name_data: "SERVER_SOFTWARE", value_data: "fastcgi-client-rs" }, ParamPair { name_length: Short(14), value_length: Short(3), name_data: "REQUEST_METHOD", value_data: "GET" },ParamPair { name_length: Short(11), value_length: Short(9), name_data: "SCRIPT_NAME", value_data: "/fpm-ping" }, ParamPair { name_length: Short(11), value_length: Short(9), name_data: "REQUEST_URI", value_data: "/fpm-ping" }, ParamPair { name_length: Short(12), value_length: Short(9), name_data: "DOCUMENT_URI", value_data: "/fpm-ping" }, ParamPair { name_length: Short(11), value_length: Short(4), name_data: "SERVER_PORT", value_data: "9000" }, ParamPair { name_length: Short(15), value_length: Short(8), name_data: "SERVER_PROTOCOL", value_data: "HTTP/1.1" }, ParamPair { name_length: Short(17), value_length: Short(11), name_data: "GATEWAY_INTERFACE", value_data: "FastCGI/1.0" }, ParamPair { name_length: Short(15), value_length: Short(9), name_data: "SCRIPT_FILENAME", value_data: "/fpm-ping" }, ParamPair { name_length: Short(11), value_length: Short(9), name_data: "SERVER_ADDR", value_data: "127.0.0.1" }])
DEBUG 2024-04-09 11:07:45 UTC: Send to stream for Params. id=1 header=Header { version: 1, type: Params, request_id: 1, content_length: 240, padding_length: 0, reserved: 0 }
DEBUG 2024-04-09 11:07:45 UTC: Send to stream for Params. id=1 header=Header { version: 1, type: Params, request_id: 1, content_length: 0, padding_length: 0, reserved: 0 }
DEBUG 2024-04-09 11:07:45 UTC: Send to stream for Stdin. id=1 header=Header { version: 1, type: Stdin, request_id: 1, content_length: 0, padding_length: 0, reserved: 0 }
DEBUG 2024-04-09 11:07:45 UTC: Send to stream for Stdin. id=1 header=Header { version: 1, type: Stdin, request_id: 1, content_length: 0, padding_length: 0, reserved: 0 }
DEBUG 2024-04-09 11:07:45 UTC: Receive from stream. id=1 header=Header { version: 1, type: Stdout, request_id: 1, content_length: 186, padding_length: 6, reserved: 0 }
DEBUG 2024-04-09 11:07:45 UTC: Receive from stream. id=1 header=Header { version: 1, type: EndRequest, request_id: 1, content_length: 8, padding_length: 0, reserved: 0 }
DEBUG 2024-04-09 11:07:45 UTC: Receive from stream. id=1 end_request_rec=EndRequestRec { header: Header { version: 1, type: EndRequest, request_id: 1, content_length: 8, padding_length: 0, reserved: 0 }, end_request: EndRequest { app_status: 0, protocol_status: RequestComplete, reserved: [0, 0, 0] } }
```

### curl call

```shell
# shell 2
curl -v http://127.0.0.1:8080/fcgi-ping
*   Trying 127.0.0.1:8080...
* Connected to 127.0.0.1 (127.0.0.1) port 8080 (#0)
> GET /fcgi-ping HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/7.81.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 0
< date: Tue, 09 Apr 2024 12:14:38 GMT
<
* Connection #0 to host 127.0.0.1 left intact
```