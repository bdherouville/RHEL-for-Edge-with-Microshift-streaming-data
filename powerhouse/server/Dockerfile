FROM rust:slim-buster as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

CMD ["./target/release/server"]

