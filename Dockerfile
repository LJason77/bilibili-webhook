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

RUN apk add -qq --update --no-cache --virtual .build-deps gcc g++ python3-dev libc-dev libffi-dev && \
    apk add -qq --update --no-cache ffmpeg python3 py3-pip sqlite-dev libc6-compat && \
    pip3 install --no-cache-dir yutto --pre --break-system-packages && \
    apk del --purge .build-deps

RUN addgroup -g 1000 pi && adduser -D -s /bin/sh -u 1000 -G pi pi && chown -R pi:pi .

USER pi

VOLUME ["/app/config", "/app/downloads"]

CMD bilibili-webhook
