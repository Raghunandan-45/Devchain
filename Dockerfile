# Dockerfile

# --- Builder Stage ---
# Use the official Rust image as a build environment
FROM rust:1.79 as builder

# Set the working directory
WORKDIR /usr/src/devchain

# Copy the source code
COPY . .

# Build the project for release in a static container
RUN cargo build --release

# --- Final Stage ---
# Use a minimal image for the final container
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/devchain/target/release/devchain_full /usr/local/bin/devchain_full

# Set the command to run when the container starts
CMD ["devchain_full"]
