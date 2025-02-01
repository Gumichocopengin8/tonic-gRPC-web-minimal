FROM rust:1.84.0-slim-bookworm AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev protobuf-compiler
WORKDIR /app
COPY ./server ./server
COPY ./proto ./proto
WORKDIR /app/server
RUN cargo build

FROM debian:bookworm-slim
RUN apt-get update && apt-get upgrade -y && apt-get install -y openssl ca-certificates
COPY --from=builder /app/server/target/debug/server /app
CMD ["/app"]
