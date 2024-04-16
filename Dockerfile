FROM rust:alpine as builder

WORKDIR /app
RUN apk add musl-dev libc6-compat openssl-dev sqlite-dev tzdata
COPY . .
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

FROM alpine:latest
LABEL org.opencontainers.image.source = "https://github.com/Felix2yu/docker_build" \
      maintainer="Felix2yu <yufei.im@icloud.com>" \
      org.opencontainers.image.authors="Felix2yu <yufei.im@icloud.com>"\
      org.opencontainers.image.authors2="ArenaDruid"

WORKDIR /app

COPY --from=builder /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
COPY --from=builder /app/target/release/bilibili-webhook /usr/local/bin/
COPY --from=builder /app/log.yml .

RUN apk add --update --no-cache --virtual .build-deps gcc g++ python3-dev libc-dev libffi-dev && \
    apk add --update --no-cache ffmpeg python3 py3-pip sqlite-dev libc6-compat && \
    pip3 install --no-cache-dir yutto --pre --break-system-packages && \
    apk del --purge .build-deps sqlite-dev && \
    adduser -D -s /bin/sh -u 1000 -G users pi && chown -R pi:users .

USER pi

VOLUME ["/app/config", "/app/downloads"]

CMD ["bilibili-webhook"]