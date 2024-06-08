FROM rust:latest

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

CMD ["./target/release/office_space_split"]
