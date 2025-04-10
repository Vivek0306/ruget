# Makefile for ruget

# Build and install locally
install-local: build
        @mkdir -p ~/.local/bin
        @cp target/release/ruget ~/.local/bin/ruget
        @echo "✅ Installed to ~/.local/bin. Make sure it's in your PATH."

# Build and install globally
install: build
        sudo cp target/release/ruget /usr/local/bin/ruget
        @echo "✅ ruget installed globally to /usr/local/bin"



# Build the release binary
build:
        cargo build --release

# Clean build artifacts
clean:
        cargo clean

# Run the binary
run: build
        ./target/release/ruget
