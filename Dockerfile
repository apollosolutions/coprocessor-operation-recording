FROM rust:1.81.0 as builder
WORKDIR /usr/src/coprocessor-operation-recording
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/coprocessor-operation-recording /usr/local/bin/coprocessor-operation-recording
ENTRYPOINT ["coprocessor-operation-recording"]
