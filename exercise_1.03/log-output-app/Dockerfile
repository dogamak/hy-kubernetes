FROM rust:1.61 AS builder

WORKDIR /usr/src/log-output-app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/log-output-app /usr/local/bin/log-output-app
CMD ["log-output-app"]
