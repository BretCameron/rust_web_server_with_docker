# Use the official Rust image as a build stage
FROM rust:1.79 AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin web_server
WORKDIR /web_server

# Copy our manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# This build step will cache dependencies
RUN cargo build --release

# Remove the dummy source file created by cargo new
RUN rm src/*.rs

# Copy source tree
COPY ./src ./src

# Build for release
RUN cargo build --release

# Use the official Debian image as a base
FROM debian:bookworm-slim

# Copy the build artifact from the build stage
COPY --from=builder /web_server/target/release/web_server /usr/local/bin/web_server

# Expose the port the server will run on
EXPOSE 5001

# Run the web service on container startup
CMD ["web_server"]
