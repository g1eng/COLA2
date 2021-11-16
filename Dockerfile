FROM rust:1.56 AS builder

RUN mkdir -vp /src/src
WORKDIR /src
COPY Cargo.toml Cargo.toml
RUN echo "fn main(){}" > src/main.rs && cargo build --release

COPY ./src ./src
RUN rm -f target/release/deps/cola2*
RUN cargo build --release

FROM debian:11.1-slim
COPY --from=builder /src/target/release/cola2 /usr/local/bin/cola2
ENTRYPOINT ["/usr/local/bin/cola2"]
