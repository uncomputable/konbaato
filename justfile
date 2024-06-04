# Build WASM package at ./pkg
wasm:
    wasm-pack build --target web

# Deploy test website at ./dist
deploy:
    just wasm
    mkdir dist
    cp pkg/konbaato_bg.wasm dist
    cp pkg/konbaato.js dist
    cp index.html dist

# Serve local test website
serve:
    just deploy
    python -m http.server 8080

# Open local website in Firefox
open:
    firefox localhost:8080

# Run unit tests in Firefox
test-wasm:
    wasm-pack test --headless --firefox

# Run unit tests on local computer
test:
    cargo test

# Run code linter
lint:
    cargo clippy --all-targets -- --deny warnings

# Format code
fmt:
    cargo fmt --all

# Check if code is formatted
fmtcheck:
    cargo fmt --all -- --check

# Check code (CI)
check:
    cargo --version
    rustc --version
    just fmtcheck
    just lint
    just test
    just test-wasm

# Run fuzzer indefinitely
fuzz:
    cargo fuzz run hirakata

# Remove all temporary files
clean:
    rm -rf target
    rm -rf pkg
    rm -rf dist
