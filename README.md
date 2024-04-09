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
