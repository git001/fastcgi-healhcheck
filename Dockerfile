FROM rust:1.77-bookworm as builder

RUN mkdir /tmp/build
COPY . /tmp/build

#RUN cargo install --path .
RUN set -x \
  && cd /tmp/build \
  && cargo build --release \ 
  && cargo install --path . \
  && cd /tmp \
  && rm -rf build \
  && pwd \
  && ls -larth \
  && ls -larth /usr/local/cargo/bin/

## Build runtime image
FROM debian:bookworm-slim

RUN set -x \
  && apt-get update \
  && apt-get install -y bash bash-completion curl \
  && apt-get -y autoremove \
  && apt-get -y autoclean \
  && rm -rf /var/lib/apt/lists/* /var/cache/apk/*

COPY --from=builder /usr/local/cargo/bin/fastcgi-healthcheck /usr/local/bin/fastcgi-healthcheck

USER 1001
EXPOSE 8080

CMD ["/usr/local/bin/fastcgi-healthcheck"]