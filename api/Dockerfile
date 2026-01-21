FROM --platform=linux/amd64 ubuntu:22.04

# ---- system deps ----
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# ---- install Rust ----
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && rustup default stable

# ---- set workdir ----
WORKDIR /zkcg

# ---- copy full monorepo ----
COPY . .

# ---- build API only ----
RUN cargo build -p api --release --features zk-vm

# ---- runtime ----
ENV PORT=8080
EXPOSE 8080

CMD ["./target/release/api"]
