#!/bin/bash

# Script helper para build Docker do projeto Axum

set -e

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Função para imprimir com cores
print_colored() {
    printf "${1}${2}${NC}\n"
}

# Função de ajuda
show_help() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  build           Build all stages"
    echo "  dev             Run development server"
    echo "  test            Run tests"
    echo "  docs            Generate and serve documentation"
    echo "  example [name]  Run specific example"
    echo "  clean           Clean Docker images and volumes"
    echo "  help            Show this help"
    echo ""
    echo "Examples:"
    echo "  $0 build"
    echo "  $0 dev"
    echo "  $0 test"
    echo "  $0 docs"
    echo "  $0 example hello-world"
    echo "  $0 clean"
}

# Função para build
build_all() {
    print_colored $BLUE "🏗️  Building Axum project..."
    
    print_colored $YELLOW "Building builder stage..."
    docker build --target builder -t axum:builder .
    
    print_colored $YELLOW "Building runtime stage..."
    docker build --target runtime -t axum:runtime .
    
    print_colored $YELLOW "Building development stage..."
    docker build --target development -t axum:dev .
    
    print_colored $GREEN "✅ Build completed!"
}

# Função para desenvolvimento
run_dev() {
    print_colored $BLUE "🚀 Starting development server..."
    docker-compose up axum-dev
}

# Função para testes
run_tests() {
    print_colored $BLUE "🧪 Running tests..."
    docker-compose up --build axum-test
}

# Função para documentação
run_docs() {
    print_colored $BLUE "📚 Generating and serving documentation..."
    print_colored $YELLOW "Documentation will be available at http://localhost:8080"
    docker-compose up --build axum-docs
}

# Função para executar exemplos
run_example() {
    local example_name=$1
    
    if [ -z "$example_name" ]; then
        print_colored $RED "❌ Please specify an example name"
        echo "Available examples:"
        ls examples/ | grep -v README.md
        return 1
    fi
    
    if [ ! -d "examples/$example_name" ]; then
        print_colored $RED "❌ Example '$example_name' not found"
        echo "Available examples:"
        ls examples/ | grep -v README.md
        return 1
    fi
    
    print_colored $BLUE "🎯 Running example: $example_name"
    
    # Build exemplo específico
    docker build -t axum:example-$example_name \
        --build-arg EXAMPLE_NAME=$example_name \
        -f- . <<EOF
FROM rust:1.75-slim as builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates
WORKDIR /app
COPY . .
RUN cargo build --release --example $example_name

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3
WORKDIR /app
COPY --from=builder /app/target/release/examples/$example_name ./app
EXPOSE 3000
CMD ["./app"]
EOF
    
    # Executar exemplo
    docker run --rm -p 3000:3000 axum:example-$example_name
}

# Função para limpeza
clean_docker() {
    print_colored $YELLOW "🧹 Cleaning Docker images and volumes..."
    
    # Remove imagens do projeto
    docker images | grep axum | awk '{print $3}' | xargs -r docker rmi -f
    
    # Remove volumes
    docker-compose down -v
    
    # Prune sistema
    docker system prune -f
    
    print_colored $GREEN "✅ Cleanup completed!"
}

# Main
case "${1:-help}" in
    build)
        build_all
        ;;
    dev)
        run_dev
        ;;
    test)
        run_tests
        ;;
    docs)
        run_docs
        ;;
    example)
        run_example "$2"
        ;;
    clean)
        clean_docker
        ;;
    help|*)
        show_help
        ;;
esac 