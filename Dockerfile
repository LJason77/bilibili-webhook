FROM rust:alpine as builder

RUN apk add -qq musl-dev libc6-compat openssl-dev sqlite-dev

WORKDIR /app

COPY Cargo* ./

RUN mkdir src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release
RUN rm -f target/release/deps/bilibili_webhook*

COPY . .

RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

FROM alpine:latest

RUN addgroup -g 1000 pi
RUN adduser -D -s /bin/sh -u 1000 -G pi pi

WORKDIR /app

COPY --from=builder /app/target/release/bilibili-webhook /usr/local/bin/
COPY log.yml .

RUN apk add -qq --no-cache libc6-compat sqlite-dev python3 py3-pip ffmpeg && \
  pip3 install --no-cache-dir bilili && \
  chown -R pi:pi .

USER pi

VOLUME ["/app/config", "/app/downloads"]

CMD bilibili-webhook
