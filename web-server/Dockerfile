FROM rust:1.78-alpine AS builder

WORKDIR /web-server
COPY . .

RUN apk update \
    && apk upgrade \
    && apk add --no-cache ca-certificates musl-dev \
    && update-ca-certificates 2>/dev/null || true

RUN cargo build --release

FROM scratch
COPY --from=builder /web-server/target/release/web-server /web-server

ENTRYPOINT ["/web-server"]
EXPOSE 8080

