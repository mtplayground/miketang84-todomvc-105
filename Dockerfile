FROM rust:1.90-bookworm AS builder

ARG CARGO_LEPTOS_VERSION=0.3.6

RUN apt-get update \
    && apt-get install -y --no-install-recommends binaryen ca-certificates pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked cargo-leptos --version ${CARGO_LEPTOS_VERSION}

WORKDIR /app

COPY . .

RUN cargo leptos build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system app \
    && useradd --system --gid app --create-home --home-dir /app app \
    && mkdir -p /app/target/site /data \
    && chown -R app:app /app /data

WORKDIR /app

ENV LEPTOS_SITE_ADDR=0.0.0.0:8080
ENV DATABASE_URL=sqlite:///data/todomvc.db
ENV RUST_LOG=info,tower_http=info

COPY --from=builder /app/target/release/miketang84-todomvc-105 /app/miketang84-todomvc-105
COPY --from=builder /app/target/site /app/target/site

EXPOSE 8080
VOLUME ["/data"]

USER app

CMD ["/app/miketang84-todomvc-105"]
