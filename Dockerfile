FROM rust:1.70 AS builder
WORKDIR /app
COPY . /app

ENV RUSTUP_DIST_SERVE https://rsproxy.cn
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.18
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ord /app

# docker run --rm -it -e RUST_LOG=info -v /data/repo/ord-regtest/data:/app/data ord:v0.6.1-api sh
# docker run --rm -p 8002:80 -e RUST_LOG=info -v /data/repo/ord-regtest/data:/app/data -v /data/repo/bitcoin-test:/app/bitcoin-test ord:v0.6.1-api sh -c ""
# docker run --rm -p 8002:80 -e RUST_LOG=info -v /data/repo/ord-regtest/data:/app/data ord:v0.6.1-api sh -c "ord -r --config=data/ord.yaml --data-dir=data --rpc-url=192.168.10.233:18443 server"