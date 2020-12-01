FROM rust:1-buster as builder

RUN USER=root cargo new --bin ddns-update
WORKDIR /ddns-update
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY . ./
RUN rm ./target/release/deps/ddns-update*
RUN cargo build --release

FROM alpine:3.12

WORKDIR /opt
COPY --from=builder /ddns-update/target/release/ddns-update ./
CMD ["./ddns-update"]