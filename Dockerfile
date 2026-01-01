# ========= build stage =========
FROM rust:1.89-bookworm AS builder
WORKDIR /usr/src/app

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates pkg-config libssl-dev clang binaryen \
 && rm -rf /var/lib/apt/lists/*

ENV CARGO_PROFILE_RELEASE_LTO=false \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
    CARGO_PROFILE_RELEASE_DEBUG=0 \
    CARGO_PROFILE_RELEASE_STRIP="symbols" \
    RUSTFLAGS="-C debuginfo=0" \
    LEPTOS_WASM_BINDGEN_VERSION=0.2.106 \
    LEPTOS_WASM_OPT=false

# cargo-leptos
RUN cargo install cargo-leptos --version 0.3.0 --locked --jobs 1
RUN rustup target add wasm32-unknown-unknown

# ★ここからは「全部COPYしてからビルド」＝更新が必ず反映される
COPY . .
RUN rm -f .cargo/config.toml .cargo/config || true

# 1) まずSSRを通す（ここで落ちたらCargo.toml側のnightlyが原因）
RUN cargo build --release --features ssr --bin plasmic_feat_genome

# 2) wasm + site を生成
RUN cargo leptos build --release

# ========= runtime stage =========
FROM debian:bookworm-slim AS runtime
WORKDIR /usr/src/app

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /usr/src/app/target/release/plasmic_feat_genome /usr/local/bin/plasmic_feat_genome
COPY --from=builder /usr/src/app/target/site /usr/src/app/target/site

EXPOSE 3000
CMD ["plasmic_feat_genome"]
