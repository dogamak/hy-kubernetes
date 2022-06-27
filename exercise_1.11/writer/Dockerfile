FROM rust:1.61 AS builder

WORKDIR /usr/src/writer
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/writer /usr/local/bin/writer
CMD ["writer"]
