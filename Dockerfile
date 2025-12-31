# ========= build stage =========
FROM rust:1.89-bookworm AS builder
WORKDIR /usr/src/app

# buildに必要な最低限（ここは apt が通る前提）
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates pkg-config libssl-dev clang binaryen \
 && rm -rf /var/lib/apt/lists/*

# cargo-leptos install を軽くする（OOM対策）
ENV CARGO_PROFILE_RELEASE_LTO=false \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
    CARGO_PROFILE_RELEASE_DEBUG=0 \
    CARGO_PROFILE_RELEASE_STRIP="symbols" \
    RUSTFLAGS="-C debuginfo=0" \
    LEPTOS_WASM_BINDGEN_VERSION=0.2.106 \
    LEPTOS_WASM_OPT=false

# manifests だけ先に（キャッシュ）
COPY Cargo.toml Cargo.lock ./

# cargo-leptos（重いので jobs=1）
RUN cargo install cargo-leptos --version 0.3.0 --locked --jobs 1

# wasm ターゲット
RUN rustup target add wasm32-unknown-unknown

# 依存だけ先にビルド（キャッシュ）
RUN rm -rf .cargo
RUN mkdir -p src \
 && printf 'fn main() {}\n' > src/main.rs \
 && printf 'pub fn dummy() {}\n' > src/lib.rs \
 && cargo build --release \
 && rm -rf src

# 本物のソース
COPY . .
RUN rm -f .cargo/config.toml .cargo/config || true

# Leptos build（SSR + wasm）
RUN cargo leptos build --release

# SSRバイナリを確実に作る
RUN cargo build --release --features ssr --bin plasmic_feat_genome

# ========= runtime stage =========
FROM debian:bookworm-slim AS runtime
WORKDIR /usr/src/app

# ★ aptしない。CAが必要なら builder から丸ごとコピーする
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# サーバーバイナリ
COPY --from=builder /usr/src/app/target/release/plasmic_feat_genome /usr/local/bin/plasmic_feat_genome

# 静的ファイル（Leptos）
COPY --from=builder /usr/src/app/target/site /usr/src/app/target/site

EXPOSE 3000
CMD ["plasmic_feat_genome"]
