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
    echo "  build           Try to build what works"
    echo "  diagnose        Run diagnostic build to see what's broken"
    echo "  dev             Run development container (interactive)"
    echo "  status          Check build status"
    echo "  download-stable Download stable Axum version"
    echo "  clean           Clean Docker images and volumes"
    echo "  help            Show this help"
    echo ""
    echo "Examples:"
    echo "  $0 build"
    echo "  $0 diagnose"
    echo "  $0 dev"
    echo "  $0 status"
    echo "  $0 download-stable"
    echo "  $0 clean"
}

# Função para build diagnóstico
build_diagnostic() {
    print_colored $BLUE "🔍 Running diagnostic build..."
    print_colored $YELLOW "This will show what works and what doesn't"
    
    docker build --target diagnostic -t axum:diagnostic .
    
    print_colored $GREEN "✅ Diagnostic build completed!"
    print_colored $BLUE "📊 Viewing diagnostic results..."
    
    docker run --rm axum:diagnostic
}

# Função para build básico
build_basic() {
    print_colored $BLUE "🏗️  Attempting to build working components..."
    print_colored $YELLOW "⚠️  This project has compatibility issues - building what we can"
    
    docker build --target builder -t axum:builder .
    docker build --target runtime -t axum:runtime .
    
    print_colored $GREEN "✅ Build attempt completed!"
    
    # Mostrar status
    show_status
}

# Função para desenvolvimento
run_dev() {
    print_colored $BLUE "🚀 Starting development container..."
    print_colored $YELLOW "This is an interactive container for investigation"
    
    docker build --target development -t axum:dev .
    docker run -it --rm -v "$(pwd):/app" -p 3000:3000 axum:dev
}

# Função para mostrar status
show_status() {
    print_colored $BLUE "📊 Checking build status..."
    
    if docker image inspect axum:runtime >/dev/null 2>&1; then
        print_colored $GREEN "✅ Runtime image exists"
        docker run --rm axum:runtime
    else
        print_colored $RED "❌ No runtime image found. Run '$0 build' first."
        return 1
    fi
}

# Função para baixar versão estável
download_stable() {
    print_colored $BLUE "📥 Downloading stable Axum version..."
    
    if [ -d "../axum-stable" ]; then
        print_colored $YELLOW "⚠️  axum-stable directory already exists"
        read -p "Remove and re-download? (y/N): " confirm
        if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
            rm -rf "../axum-stable"
        else
            print_colored $BLUE "Using existing axum-stable directory"
            return 0
        fi
    fi
    
    cd ..
    git clone https://github.com/tokio-rs/axum.git axum-stable
    cd axum-stable
    
    print_colored $YELLOW "📋 Available stable tags:"
    git tag | grep "^v0\." | tail -5
    
    # Usar versão mais recente estável
    latest_tag=$(git tag | grep "^v0\." | tail -1)
    print_colored $BLUE "Checking out latest stable: $latest_tag"
    git checkout "$latest_tag"
    
    print_colored $GREEN "✅ Stable version ready at ../axum-stable"
    print_colored $BLUE "💡 To use: cd ../axum-stable && docker build ."
}

# Função para limpeza
clean_docker() {
    print_colored $YELLOW "🧹 Cleaning Docker images and volumes..."
    
    # Remove imagens do projeto
    docker images | grep axum | awk '{print $3}' | xargs -r docker rmi -f
    
    # Remove volumes se existirem
    docker-compose down -v 2>/dev/null || true
    
    # Prune sistema
    docker system prune -f
    
    print_colored $GREEN "✅ Cleanup completed!"
}

# Função para investigar problemas
investigate() {
    print_colored $BLUE "🔍 Investigating project structure..."
    
    echo "=== Project Analysis ==="
    echo "Workspace members:"
    grep -A 10 "\\[workspace\\]" Cargo.toml 2>/dev/null || echo "No workspace found"
    
    echo ""
    echo "Crate dependencies:"
    find . -name "Cargo.toml" -exec echo "=== {} ===" \; -exec head -15 {} \; | head -50
    
    echo ""
    echo "Rust version required:"
    grep "rust-version" Cargo.toml 2>/dev/null || echo "No rust-version specified"
}

# Main
case "${1:-help}" in
    build)
        build_basic
        ;;
    diagnose)
        build_diagnostic
        ;;
    dev)
        run_dev
        ;;
    status)
        show_status
        ;;
    download-stable)
        download_stable
        ;;
    investigate)
        investigate
        ;;
    clean)
        clean_docker
        ;;
    help|*)
        show_help
        ;;
esac 