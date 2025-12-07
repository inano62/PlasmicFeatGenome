# ========= build stage =========
FROM rust:1.84 AS builder

WORKDIR /usr/src/app

# cargo-leptos をインストール（0.3.0に固定）
RUN cargo install cargo-leptos --version 0.3.0 --locked

# 依存解決キャッシュ
# Cargo.lock が無い場合は、この行を「COPY Cargo.toml ./」だけにする
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

# ビルド成果物だけコピー
COPY --from=builder /usr/src/app/target/release/plasmic_feat_genome /usr/local/bin/plasmic_feat_genome
COPY --from=builder /usr/src/app/target/site /usr/src/app/target/site

EXPOSE 3000

CMD ["plasmic_feat_genome"]
