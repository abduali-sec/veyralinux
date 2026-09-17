#!/bin/bash

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT/veyra-cli"

cargo build --release --bin vpm

install -Dm755 \
    "$ROOT/veyra-cli/target/release/vpm" \
    "$ROOT/veyra-iso/airootfs/usr/bin/vpm"

echo "✓ Veyra vpm copied into ISO profile"
