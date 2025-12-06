FROM rust:1.84-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       pkg-config libssl-dev openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# 依存解決キャッシュ用
COPY Cargo.toml ./
RUN mkdir src \
    && echo "fn main() { println!(\"dummy\"); }" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# 本物のソース
COPY . .

# Axum サーバ起動
CMD ["cargo", "run", "--release"]
