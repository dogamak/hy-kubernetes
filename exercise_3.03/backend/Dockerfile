FROM rust:1.61 AS builder

WORKDIR /usr/src/todo-app
COPY . .
RUN cargo build --all --release

FROM debian:buster-slim AS todo-app
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates
COPY --from=builder /usr/src/todo-app/target/release/todo-app /usr/local/bin/todo-app

CMD ["todo-app"]

FROM debian:buster-slim AS daily-todo-job
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates
COPY --from=builder /usr/src/todo-app/target/release/daily-todo-job /usr/local/bin/daily-todo-job

CMD ["daily-todo-job"]
