#!/usr/bin/env bash
# Build, test, and install the `ee` Rust CLI from tools/ee into ~/.cargo/bin/ee.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

cargo test --offline
cargo build --release --offline

mkdir -p "${HOME}/.cargo/bin"
install -m 0755 "target/release/ee" "${HOME}/.cargo/bin/ee"
echo "installed: ${HOME}/.cargo/bin/ee ($(sha256sum "${HOME}/.cargo/bin/ee" | awk '{print $1}'))"
