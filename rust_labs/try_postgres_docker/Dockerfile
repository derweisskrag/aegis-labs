# Build stage
FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .

# This will build the 'everglades' binary specifically
RUN cargo build --release --bin everglades

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Note the path: it's /app/target/release/everglades
COPY --from=builder /app/target/release/everglades /app/everglades

CMD ["./everglades"]