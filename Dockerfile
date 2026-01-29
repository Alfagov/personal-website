# Stage 1: Build the application
FROM rust:1.84-alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/portfolio

# Copy manifest files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src

# Copy the actual source code and templates
COPY src ./src
COPY templates ./templates
COPY static ./static

# Build the actual application
RUN cargo build --release

# Stage 2: Create the runtime image
FROM alpine

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/portfolio/target/release/portfolio .

# Copy static assets (required for ServeDir)
COPY static ./static

# Expose the port the app runs on
EXPOSE 3000

# Run the application
CMD ["./portfolio"]
