#!/bin/bash
#
# Archive old results (> X days)
# Run via cron: 0 4 * * 0 ~/projects/perso/dark-gpt/scripts/cleanup-results.sh
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RESULTS_DIR="$PROJECT_ROOT/results"
ARCHIVE_DIR="$PROJECT_ROOT/archive"

# Configuration
RETENTION_DAYS="${RETENTION_DAYS:-30}"
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
echo "📦 Dark-GPT Results Archive"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Source: $RESULTS_DIR"
echo "Archive: $ARCHIVE_DIR"
echo "Retention: $RETENTION_DAYS days"
echo "Dry Run: $DRY_RUN"
echo ""

if [[ ! -d "$RESULTS_DIR" ]]; then
    log_warn "Results directory not found: $RESULTS_DIR"
    exit 0
fi

# Find old result files
old_files=$(find "$RESULTS_DIR" -type f \( -name "*.jsonl" -o -name "*.json" -o -name "*.csv" \) -mtime +$RETENTION_DAYS 2>/dev/null || true)

if [[ -z "$old_files" ]]; then
    log_success "No files older than $RETENTION_DAYS days"
    exit 0
fi

file_count=$(echo "$old_files" | wc -l)
log_info "Found $file_count files to archive"

# Create archive directory
archive_date=$(date +%Y-%m-%d)
archive_target="$ARCHIVE_DIR/results-$archive_date"

if [[ "$DRY_RUN" != "true" ]]; then
    mkdir -p "$archive_target"
fi

# List and archive files
echo ""
echo "Files to archive:"
total_size=0
while IFS= read -r file; do
    if [[ -n "$file" ]]; then
        size=$(stat --format=%s "$file" 2>/dev/null || echo 0)
        total_size=$((total_size + size))
        rel_path="${file#$RESULTS_DIR/}"
        echo "  - $rel_path ($(numfmt --to=iec $size 2>/dev/null || echo "${size}B"))"

        if [[ "$DRY_RUN" != "true" ]]; then
            # Preserve directory structure
            target_dir="$archive_target/$(dirname "$rel_path")"
            mkdir -p "$target_dir"
            mv "$file" "$target_dir/"
        fi
    fi
done <<< "$old_files"

echo ""
echo "Total size: $(numfmt --to=iec $total_size 2>/dev/null || echo "${total_size}B")"

# Compress archive
if [[ "$DRY_RUN" != "true" && -d "$archive_target" ]]; then
    log_info "Compressing archive..."
    cd "$ARCHIVE_DIR"
    tar -czf "results-$archive_date.tar.gz" "results-$archive_date"
    rm -rf "results-$archive_date"
    log_success "Created: $ARCHIVE_DIR/results-$archive_date.tar.gz"
else
    log_warn "DRY RUN - No files archived"
fi

# Cleanup empty directories
if [[ "$DRY_RUN" != "true" ]]; then
    find "$RESULTS_DIR" -type d -empty -delete 2>/dev/null || true
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Archive Complete"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
