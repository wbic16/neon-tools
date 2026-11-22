#!/bin/bash
# Build phext-write on Neon's box and install it next to phext-tool.
# Zero-token local work; the libphext path dependency keeps it offline.
set -e
cd "$(dirname "$0")"
export PATH=$HOME/.cargo/bin:$PATH
cargo build --release --offline 2>/dev/null || cargo build --release
install -m 755 target/release/phext-write "$HOME/.cargo/bin/phext-write"
echo "installed: $($HOME/.cargo/bin/phext-write 2>&1 | head -1 || true)"
phext-write 2>&1 | grep -q usage && echo "phext-write OK in ~/.cargo/bin"
