#!/bin/bash
#
# Setup HTTPS certificates for Dark-GPT
# Uses mkcert for local development certificates
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CERTS_DIR="$PROJECT_ROOT/docker/certs"
DOMAIN="dark-gpt.local"

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

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔐 Dark-GPT HTTPS Setup"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check mkcert
if ! command -v mkcert &> /dev/null; then
    log_error "mkcert not found. Install it first:"
    echo "  curl -JLO 'https://dl.filippo.io/mkcert/latest?for=linux/amd64'"
    echo "  chmod +x mkcert-v*-linux-amd64"
    echo "  sudo mv mkcert-v*-linux-amd64 /usr/local/bin/mkcert"
    exit 1
fi

log_success "mkcert found: $(mkcert --version)"

# Install local CA
log_info "Installing local CA..."
mkcert -install
log_success "Local CA installed"

# Create certs directory
mkdir -p "$CERTS_DIR"

# Generate certificates
log_info "Generating certificates for $DOMAIN..."
cd "$CERTS_DIR"
mkcert -cert-file "$DOMAIN.pem" -key-file "$DOMAIN-key.pem" "$DOMAIN" localhost 127.0.0.1 ::1
log_success "Certificates generated in $CERTS_DIR"

# Add to /etc/hosts if not present
if ! grep -q "$DOMAIN" /etc/hosts 2>/dev/null; then
    log_info "Adding $DOMAIN to /etc/hosts (requires sudo)..."
    echo "127.0.0.1 $DOMAIN" | sudo tee -a /etc/hosts > /dev/null
    log_success "Added $DOMAIN to /etc/hosts"
else
    log_success "$DOMAIN already in /etc/hosts"
fi

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ HTTPS Setup Complete"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Certificates:"
ls -la "$CERTS_DIR"/*.pem
echo ""
echo "Access: https://$DOMAIN"
echo ""
