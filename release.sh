#!/bin/bash
# Release script for work CLI tool

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }

# Check if version is provided
if [ -z "$1" ]; then
    error "Usage: $0 <version> (e.g., $0 v0.1.0)"
fi

VERSION=$1

# Validate version format
if [[ ! $VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    error "Version must be in format v0.1.0 (e.g., v0.1.0, v1.2.3)"
fi

# Remove 'v' prefix for Cargo
CARGO_VERSION="${VERSION#v}"

info "Preparing release $VERSION..."

# Update Cargo.toml
info "Updating Cargo.toml version to $CARGO_VERSION"
sed -i.bak "s/^version = \".*\"/version = \"$CARGO_VERSION\"/" Cargo.toml
rm -f Cargo.toml.bak

# Update src/main.rs version
info "Updating src/main.rs version to $CARGO_VERSION"
sed -i.bak "s/\[command(version = \".*\")\]/[command(version = \"$CARGO_VERSION\")]/" src/main.rs
rm -f src/main.rs.bak

# Run tests
info "Running tests..."
if ! cargo test; then
    error "Tests failed! Aborting release."
fi

# Build release binary
info "Building release binary..."
if ! cargo build --release; then
    error "Build failed! Aborting release."
fi

# Create git commit
info "Creating git commit..."
git add Cargo.toml src/main.rs
git commit -m "Release $VERSION"

# Create git tag
info "Creating git tag $VERSION..."
git tag -a "$VERSION" -m "Release $VERSION"

info "Release $VERSION prepared successfully!"
info ""
info "Next steps:"
info "1. Review the changes: git log -1"
info "2. Push to GitHub: git push && git push --tags"
info "3. GitHub Actions will build and create the release automatically"
warn "Make sure to update the Homebrew formula version after release!"
