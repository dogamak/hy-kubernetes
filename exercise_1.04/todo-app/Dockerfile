FROM rust:1.61 AS builder

WORKDIR /usr/src/todo-app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/todo-app /usr/local/bin/todo-app
CMD ["todo-app"]
