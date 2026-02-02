#!/bin/bash
#
# Dark-GPT Deployment Script
# Orchestrates complete deployment with health checks
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DOCKER_DIR="$PROJECT_ROOT/docker"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${CYAN}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

show_banner() {
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}🚀 Dark-GPT Deployment${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

check_prerequisites() {
    log_info "Checking prerequisites..."

    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker not found"
        exit 1
    fi
    log_success "Docker found"

    # Check Ollama running on host
    if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
        log_success "Ollama running on host"
    else
        log_warn "Ollama not running on host (required for WebUI)"
        echo "  Start with: ollama serve"
    fi

    # Check certificates
    if [[ ! -f "$DOCKER_DIR/certs/dark-gpt.local.pem" ]]; then
        log_warn "HTTPS certificates not found"
        echo "  Run: ./scripts/setup-https.sh"

        read -p "Setup HTTPS now? [Y/n] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Nn]$ ]]; then
            "$SCRIPT_DIR/setup-https.sh"
        else
            exit 1
        fi
    else
        log_success "HTTPS certificates found"
    fi

    # Check .env
    if [[ ! -f "$DOCKER_DIR/.env" ]]; then
        log_error ".env file not found"
        echo "  Copy and configure: cp docker/.env.example docker/.env"
        exit 1
    fi
    log_success ".env file found"

    # Check /etc/hosts
    if ! grep -q "dark-gpt.local" /etc/hosts 2>/dev/null; then
        log_warn "dark-gpt.local not in /etc/hosts"
        echo "127.0.0.1 dark-gpt.local" | sudo tee -a /etc/hosts > /dev/null
        log_success "Added dark-gpt.local to /etc/hosts"
    else
        log_success "dark-gpt.local in /etc/hosts"
    fi
}

deploy() {
    log_info "Deploying Dark-GPT..."

    cd "$DOCKER_DIR"

    # Pull latest images
    log_info "Pulling latest images..."
    docker compose pull

    # Stop existing containers
    log_info "Stopping existing containers..."
    docker compose down --remove-orphans 2>/dev/null || true

    # Start services
    log_info "Starting services..."
    docker compose up -d

    # Wait for health
    log_info "Waiting for services to be healthy..."
    sleep 5

    local max_attempts=30
    local attempt=0
    while [[ $attempt -lt $max_attempts ]]; do
        if curl -sk https://dark-gpt.local/health > /dev/null 2>&1; then
            break
        fi
        attempt=$((attempt + 1))
        echo -n "."
        sleep 2
    done
    echo ""

    if [[ $attempt -eq $max_attempts ]]; then
        log_error "Services failed to become healthy"
        docker compose logs --tail=50
        exit 1
    fi

    log_success "Services healthy"
}

show_status() {
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}✅ Deployment Complete${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "Containers:"
    docker ps --format "  {{.Names}}: {{.Status}}" | grep dark-gpt
    echo ""
    echo "Access:"
    echo "  🌐 https://dark-gpt.local"
    echo ""
    echo "First user to register becomes admin!"
    echo ""
}

# Main
show_banner
check_prerequisites
deploy
show_status
