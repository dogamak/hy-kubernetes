FROM rust:1.61 AS builder

WORKDIR /usr/src/log-output-app
COPY . .
RUN --mount=type=cache,target=/root/.cargo cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates libssl1.1
COPY --from=builder /usr/local/cargo/bin/log-output-app /usr/local/bin/log-output-app
CMD ["log-output-app"]
