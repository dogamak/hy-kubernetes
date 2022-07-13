FROM rust:1.61 AS builder

WORKDIR /usr/src/ping-pong-app
COPY . .
RUN --mount=type=cache,target=/root/.cargo cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/ping-pong-app /usr/local/bin/ping-pong-app
CMD ["ping-pong-app"]
