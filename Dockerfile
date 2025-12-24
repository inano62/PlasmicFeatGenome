# ========= build stage =========
FROM rust:1.89 AS builder

WORKDIR /usr/src/app

# cargo-leptos を先に入れてキャッシュさせる
RUN cargo install cargo-leptos --version 0.3.0 --locked

# 依存解決キャッシュ
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
 && echo "fn main() {}" > src/main.rs \
 && cargo build --release \
 && rm -rf src

# 本物のソース
COPY . .

# Leptos の SSR + Client(JS/WASM) をビルド
RUN cargo leptos build --release

# ========= runtime stage =========
FROM debian:stable-slim AS runtime

WORKDIR /usr/src/app

# 必要なCA証明書（reqwest/https用）
RUN apt-get update \
 && apt-get install -y ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# ビルド成果物だけコピー
COPY --from=builder /usr/src/app/target/release/* /usr/local/bin/
COPY --from=builder /usr/src/app/target/site /usr/src/app/target/site

EXPOSE 3000

CMD ["plasmic_feat_genome"]
