FROM rust:1.61 AS builder

WORKDIR /usr/src/reader
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/reader /usr/local/bin/reader
CMD ["reader"]
