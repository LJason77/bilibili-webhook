FROM rust:alpine AS builder

RUN --mount=type=cache,target=/var/cache/apk,sharing=locked,id=alpine_apk apk add -qq --update-cache --repository=https://dl-cdn.alpinelinux.org/alpine/edge/community musl-dev libc6-compat openssl-dev sqlite-dev tzdata

WORKDIR /app

COPY . .

ARG CARGO_BUILD_JOBS=4
ARG CARGO_BUILD_RUSTFLAGS="-C target-cpu=native"

RUN --mount=type=cache,target=/usr/local/cargo/git,id=alpine_cargo_git \
    --mount=type=cache,target=/usr/local/cargo/registry,id=alpine_cargo_registry \
    --mount=type=cache,target=/app/target,id=alpine_cargo \
    cargo build --release -q && \
    cp target/release/bilibili-webhook .

FROM alpine:latest AS app

WORKDIR /app

RUN --mount=type=cache,target=/var/cache/apk,sharing=locked,id=alpine_apk \
    --mount=type=cache,target=/root/.cache/pip,id=pip \
    apk add -qq --update-cache --virtual .build-deps gcc g++ python3-dev libc-dev libffi-dev && \
    apk add -qq --update-cache ffmpeg python3 py3-pip sqlite-dev libc6-compat && \
    pip3 install yutto --cache-dir /root/.cache/pip -U --break-system-packages && \
    apk del --purge .build-deps

RUN addgroup -g 1000 pi && adduser -D -s /bin/sh -u 1000 -G pi pi && chown -R pi:pi .

COPY --from=builder /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
COPY --from=builder /app/bilibili-webhook /usr/local/bin/
COPY log.yml .

USER pi

VOLUME ["/app/config", "/app/downloads"]

CMD ["bilibili-webhook"]
