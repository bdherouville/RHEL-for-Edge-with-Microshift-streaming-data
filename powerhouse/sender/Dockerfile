FROM rust
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
CMD ["./target/release/csv-to-rest"]

