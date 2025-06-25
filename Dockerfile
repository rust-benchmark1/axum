# Dockerfile multi-stage para projeto Axum
# Stage 1: Builder
FROM rust:1.75-slim as builder

# Instalar dependências do sistema necessárias
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Definir diretório de trabalho
WORKDIR /app

# Copiar arquivos de configuração Rust
COPY Cargo.toml Cargo.lock ./
COPY deny.toml .clippy.toml ./

# Copiar todos os crates do workspace
COPY axum/ ./axum/
COPY axum-core/ ./axum-core/
COPY axum-extra/ ./axum-extra/
COPY axum-macros/ ./axum-macros/

# Criar dummy src para cache das dependências
RUN mkdir -p axum/src axum-core/src axum-extra/src axum-macros/src && \
    echo "fn main() {}" > axum/src/main.rs && \
    echo "// dummy" > axum-core/src/lib.rs && \
    echo "// dummy" > axum-extra/src/lib.rs && \
    echo "// dummy" > axum-macros/src/lib.rs

# Build apenas as dependências (para cache)
RUN cargo build --release --workspace
RUN rm -rf axum/src axum-core/src axum-extra/src axum-macros/src

# Copiar código fonte real
COPY axum/ ./axum/
COPY axum-core/ ./axum-core/
COPY axum-extra/ ./axum-extra/
COPY axum-macros/ ./axum-macros/

# Build final do projeto
RUN cargo build --release --workspace

# Stage 2: Runtime (para aplicações)
FROM debian:bookworm-slim as runtime

# Instalar dependências de runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Criar usuário não-root
RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

# Copiar binários compilados
COPY --from=builder /app/target/release/ ./bin/

# Trocar para usuário não-root
USER appuser

# Stage 3: Development
FROM rust:1.75-slim as development

# Instalar dependências do sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# Instalar ferramentas úteis para desenvolvimento
RUN cargo install cargo-watch cargo-expand

WORKDIR /app

# Copiar projeto
COPY . .

# Expor porta padrão para desenvolvimento
EXPOSE 3000

# Comando padrão para desenvolvimento
CMD ["cargo", "watch", "-x", "run"]

# Stage 4: Test
FROM builder as test

WORKDIR /app

# Executar testes
RUN cargo test --workspace

# Stage 5: Documentation
FROM builder as docs

WORKDIR /app

# Gerar documentação
RUN cargo doc --workspace --no-deps

# Stage 6: Examples (para rodar exemplos específicos)
FROM runtime as examples

# Copiar exemplos compilados
COPY --from=builder /app/examples/ ./examples/

# Expor porta para exemplos
EXPOSE 3000

# Ponto de entrada flexível para diferentes exemplos
ENTRYPOINT ["./examples/"] 