# Use the official Rust image as the base image
FROM rust:latest

# Create a working directory for the project
WORKDIR /usr/src/qa

# Copy the project files to the container
COPY . .

# Build the project
RUN cargo build --release

# Set the entry point for the container
ENTRYPOINT ["./target/release/qa"]