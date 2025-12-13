#!/usr/bin/env bash
set -e

echo "Building morse (Morse language compiler/interpreter)..."
cargo build --release

echo ""
echo "Installing binary..."

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    sudo cp target/release/morse /usr/local/bin/morse
    echo "Installed to /usr/local/bin/morse"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    sudo cp target/release/morse /usr/local/bin/morse
    echo "Installed to /usr/local/bin/morse"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    echo "Windows detected. Please manually copy the binary:"
    echo "  copy target\\release\\morse.exe C:\\Windows\\"
    echo "Or add target\\release to your PATH"
    exit 0
else
    echo "Unknown OS. Please manually install target/release/morse to your PATH"
    exit 1
fi

echo ""
echo "Cleaning build artifacts..."
cargo clean

echo ""
echo "Installation complete!"
echo ""
echo "Try these commands:"
echo "  morse run example.mo"
echo "  morse build example.mo -o hello"
echo "  ./hello"
