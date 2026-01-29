# Stage 1: Build the application
FROM rust:1.84-alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/portfolio

# Copy manifest files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Install Node.js and npm to build Tailwind CSS
RUN apk add nodejs npm

# Create a dummy main.rs to build dependencies
RUN mkdir src

# Copy the actual source code and templates
COPY src ./src
COPY templates ./templates
COPY package.json package-lock.json tailwind.config.js ./
COPY static ./static

# Install dependencies and build Tailwind CSS
RUN npm install
RUN npm run build:css

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
