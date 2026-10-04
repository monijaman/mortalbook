FROM rust:1-slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

FROM debian:stable-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/mortalbook-backend /usr/local/bin/mortalbook-backend
ENV UPLOAD_DIR=/data/uploads
EXPOSE 8080
CMD ["mortalbook-backend"]
