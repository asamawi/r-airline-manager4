# Start from an Alpine-based Rust image for musl support
FROM rust:alpine AS builder

# Install musl, OpenSSL (with static libraries), and pkg-config
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

# Set environment variables for static linking
ENV OPENSSL_STATIC=1
ENV OPENSSL_DIR=/usr
ENV PKG_CONFIG_ALLOW_CROSS=1

# Set the working directory
WORKDIR /usr/src/app

# Copy the project files into the container
COPY . .

# Add the musl target and build the project in release mode
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# Use a scratch image for the final, minimal container
FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/airline_manager4 /bin/airline_manager4
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Set the command to run the application
CMD ["/bin/airline_manager4"]
