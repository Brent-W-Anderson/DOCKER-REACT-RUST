# Use the official Rust image
FROM rust:1.72.1

# Set the working directory
WORKDIR /rust-backend

# Update Cargo to the latest stable version
RUN rustup update stable && rustup install stable && rustup default stable

# Copy the Cargo files and a minimal src directory to allow dependency fetching
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Fetch dependencies
RUN cargo fetch

# Copy the remaining project files
COPY . .

# Build the application
RUN cargo build --release

# Expose port 8080
EXPOSE 8080

# Run the compiled binary
CMD ["./target/release/rust-backend"]
