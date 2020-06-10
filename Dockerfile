# Builder image
FROM rust:latest as builder

WORKDIR /build

COPY ./src /build/src
COPY ./Cargo.toml /build/Cargo.toml

RUN cargo build --release

# Exec image
FROM rust:slim-buster

WORKDIR /app

COPY --from=builder /build/target/release/blog /app/blog
COPY ./posts /app/posts
COPY ./public /app/public
COPY ./templates /app/templates

CMD ["/app/blog"]
