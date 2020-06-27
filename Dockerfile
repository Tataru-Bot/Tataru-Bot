FROM rust:1.44 as builder
WORKDIR /usr/src/tataru
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/tataru /usr/local/bin/tataru
CMD ["tataru"]