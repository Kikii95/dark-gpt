#!/bin/bash
#
# Cleanup old logs (> X days)
# Run via cron: 0 3 * * * ~/projects/perso/dark-gpt/scripts/cleanup-logs.sh
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LOGS_DIR="$PROJECT_ROOT/logs"

# Configuration
RETENTION_DAYS="${RETENTION_DAYS:-7}"
DRY_RUN="${DRY_RUN:-false}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${CYAN}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🗑️  Dark-GPT Log Cleanup"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Directory: $LOGS_DIR"
echo "Retention: $RETENTION_DAYS days"
echo "Dry Run: $DRY_RUN"
echo ""

if [[ ! -d "$LOGS_DIR" ]]; then
    log_warn "Logs directory not found: $LOGS_DIR"
    exit 0
fi

# Find old files
old_files=$(find "$LOGS_DIR" -type f \( -name "*.jsonl" -o -name "*.json" -o -name "*.log" \) -mtime +$RETENTION_DAYS 2>/dev/null || true)

if [[ -z "$old_files" ]]; then
    log_success "No files older than $RETENTION_DAYS days"
    exit 0
fi

file_count=$(echo "$old_files" | wc -l)
log_info "Found $file_count files to delete"

# List files
echo ""
echo "Files to delete:"
echo "$old_files" | while read -r file; do
    size=$(du -h "$file" 2>/dev/null | cut -f1)
    echo "  - $file ($size)"
done
echo ""

# Delete or dry-run
if [[ "$DRY_RUN" == "true" ]]; then
    log_warn "DRY RUN - No files deleted"
else
    echo "$old_files" | xargs -r rm -f
    log_success "Deleted $file_count files"
fi

# Cleanup Docker logs (Caddy)
if docker ps -q --filter "name=dark-gpt-caddy" > /dev/null 2>&1; then
    log_info "Truncating Caddy container logs..."
    if [[ "$DRY_RUN" != "true" ]]; then
        docker exec dark-gpt-caddy sh -c "find /var/log/caddy -name '*.log' -mtime +$RETENTION_DAYS -delete 2>/dev/null || true"
        log_success "Caddy logs cleaned"
    fi
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Cleanup Complete"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
