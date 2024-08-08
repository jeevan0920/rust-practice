# Use the official Rust image as a builder
FROM rust:1.64 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main file to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release && rm -rf src/target/release/deps/cpu_info*

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a lightweight image to run the application
FROM debian:buster-slim

# Copy the built application from the builder
COPY --from=builder /usr/src/app/target/release/cpu_info /usr/local/bin/cpu_info

# Set the startup command
CMD ["cpu_info"]
