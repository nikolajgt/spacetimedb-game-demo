FROM rust:1.86 as builder
WORKDIR /server
COPY . .

# Install CLI
RUN cargo install spacetimedb-cli

# Build the module
RUN spacetimedb build

# Runtime image
FROM debian:bookworm-slim
COPY --from=builder /server /server
WORKDIR /server

# Expose SpaceTimeDB port
EXPOSE 3000

CMD ["spacetimedb", "serve", "game-demo", "--bind", "0.0.0.0:3000"]


