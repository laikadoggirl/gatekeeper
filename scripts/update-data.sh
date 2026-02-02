#!/usr/bin/env bash
# Update vendored data files for tirith.
# Run from the repository root.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DATA_DIR="${SCRIPT_DIR}/../data"

echo "Updating vendored data files..."

# Update Public Suffix List
echo "  Fetching public_suffix_list.dat..."
curl -sSL "https://publicsuffix.org/list/public_suffix_list.dat" \
  -o "${DATA_DIR}/public_suffix_list.dat"
echo "  Done."

# Update Unicode confusables
echo "  Fetching confusables.txt..."
curl -sSL "https://www.unicode.org/Public/security/latest/confusables.txt" \
  -o "${DATA_DIR}/confusables_full.txt"
echo "  Done."

echo ""
echo "Note: known_domains.csv and popular_repos.csv are manually curated."
echo "Review and update them as needed."
echo ""
echo "After updating, run 'cargo build' to regenerate embedded data."
