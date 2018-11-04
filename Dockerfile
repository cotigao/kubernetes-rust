FROM rust:1.29.2

WORKDIR /app
COPY Cargo.toml /app/Cargo.toml
COPY src/ /app/src
COPY k8s/ /app/k8s
COPY examples/ /app/examples

# RUN cargo build --examples

ENTRYPOINT ["sleep", "infinity"]
