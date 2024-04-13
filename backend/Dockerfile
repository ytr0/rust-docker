ARG RUST_VERSION=latest
FROM rust:${RUST_VERSION}

WORKDIR /app

# COPY entrypoint.sh /app/entrypoint.sh
# RUN chmod +x /app/entrypoint.sh

# Install musl-tools to make many crates compile successfully
# RUN apk add --no-cache musl-dev

# Install cargo-watch
# RUN cargo install cargo-watch


COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]