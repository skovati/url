FROM ekidd/rust-musl-builder:stable as build

RUN USER=root cargo new --bin url
WORKDIR ./url
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/url*
RUN cargo build --release

FROM alpine:latest

RUN apk update \
    && apk add --no-cache ca-certificates

COPY --from=builder /home/rust/src/url/target/x86_64-unknown-linux-musl/release/url /app

CMD ["./url"]
