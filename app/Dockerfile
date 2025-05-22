# Stage 1: Build
FROM rust:1.87 AS builder
WORKDIR /usr/src/m321_app
COPY . .
RUN cargo install --path .

# Stage 2: Run
FROM debian:bookworm-slim
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/m321_app /usr/local/bin/m321_app
COPY --from=builder /usr/src/m321_app/static /static
CMD m321_app
