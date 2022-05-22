FROM rust:alpine as builder

RUN apk add -qq --repository=https://dl-cdn.alpinelinux.org/alpine/edge/community musl-dev libc6-compat openssl-dev sqlite-dev tzdata

WORKDIR /app

COPY . .

RUN RUSTFLAGS="-C target-cpu=native" cargo build --release -q

FROM alpine:latest

WORKDIR /app

COPY --from=builder /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
COPY --from=builder /app/target/release/bilibili-webhook /usr/local/bin/
COPY log.yml .

RUN apk add -qq --no-cache libc6-compat sqlite-dev python3 python3-dev py3-pip ffmpeg gcc libc-dev && \
    pip3 install --no-cache-dir yutto --pre

RUN addgroup -g 1000 pi && adduser -D -s /bin/sh -u 1000 -G pi pi && chown -R pi:pi .

USER pi

VOLUME ["/app/config", "/app/downloads"]

CMD bilibili-webhook
