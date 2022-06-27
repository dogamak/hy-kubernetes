FROM rust:1.61 AS builder

WORKDIR /usr/src/todo-app
COPY backend .
RUN cargo install --path .

FROM node:14-alpine AS frontend

WORKDIR /app
COPY frontend/package.json .
COPY frontend/package-lock.json .
RUN yarn install --production
COPY frontend .
RUN npm run build

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates
COPY --from=builder /usr/local/cargo/bin/todo-app /usr/local/bin/todo-app
COPY --from=frontend /app/build /static

CMD ["todo-app"]
