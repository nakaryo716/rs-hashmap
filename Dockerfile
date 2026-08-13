FROM rust:1.97.1
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
COPY examples ./examples
RUN cargo build --release --all-targets

CMD [ "cargo", "test" ]
