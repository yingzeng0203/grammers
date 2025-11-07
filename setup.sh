#!/bin/bash
set -e

echo "=== Grammers Telegram Client Setup ==="
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✓ Rust already installed: $(rustc --version)"
fi

echo

# Build the project
echo "Building Grammers project..."
cargo build --release

echo
echo "✓ Build successful!"
echo

# Run demo
echo "Running demo application..."
echo "---"
cargo run --release
echo "---"
echo

echo "✓ Setup complete!"
echo
echo "Next steps:"
echo "1. Read SETUP_GUIDE.md for detailed instructions"
echo "2. Get API credentials from https://my.telegram.org/apps"
echo "3. Modify grammers/src/main.rs to add your credentials"
echo "4. Run: cargo run --release"
echo
echo "Available examples:"
echo "  cargo run --example ping --release"
echo "  cargo run --example echo --release"
echo "  cargo run --example dialogs --release"
echo "  cargo run --example downloader --release"
