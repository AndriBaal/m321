# Stage 1: Build
FROM rust:1.85 AS builder
WORKDIR /usr/src/m324_app
COPY . .
RUN cargo install --path .

# Stage 2: Run
FROM debian:bookworm-slim
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/m324_app /usr/local/bin/m324_app
COPY --from=builder /usr/src/m324_app/static /static
CMD m324_app
