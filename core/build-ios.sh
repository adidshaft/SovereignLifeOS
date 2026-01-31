#!/bin/bash
set -e

# Directory setup
cd "$(dirname "$0")"
CORE_DIR=$(pwd)
IOS_LIBS_DIR="../ios_libs"
mkdir -p "${IOS_LIBS_DIR}"

echo "🚀 Starting iOS Build Process..."

# Install targets
echo "📦 Installing Rust targets..."
rustup target add aarch64-apple-ios aarch64-apple-ios-sim

# Build for targets
echo "🛠 Building for aarch64-apple-ios..."
cargo build --release --target aarch64-apple-ios
echo "🛠 Building for aarch64-apple-ios-sim..."
cargo build --release --target aarch64-apple-ios-sim

# Generate Swift bindings first (needed for headers)
echo "📝 Generating Swift bindings..."
echo "🛠 Building host library for binding generation..."
cargo build --release # Builds for host (macOS)
HOST_LIB="target/release/libsovereign_core.dylib"

cargo run --bin uniffi-bindgen generate \
    --library \
    "${HOST_LIB}" \
    --language swift \
    --out-dir "${IOS_LIBS_DIR}"

# Prepare headers for XCFramework
echo "📂 Preparing headers..."
HEADERS_DIR="${IOS_LIBS_DIR}/Headers"
mkdir -p "${HEADERS_DIR}"
mv "${IOS_LIBS_DIR}/sovereign_coreFFI.h" "${HEADERS_DIR}/"
mv "${IOS_LIBS_DIR}/sovereign_coreFFI.modulemap" "${HEADERS_DIR}/module.modulemap"

# Create XCFramework
echo "📦 Creating XCFramework..."
rm -rf "${IOS_LIBS_DIR}/SovereignCore.xcframework"

xcodebuild -create-xcframework \
    -library "target/aarch64-apple-ios/release/libsovereign_core.a" \
    -headers "${HEADERS_DIR}" \
    -library "target/aarch64-apple-ios-sim/release/libsovereign_core.a" \
    -headers "${HEADERS_DIR}" \
    -output "${IOS_LIBS_DIR}/SovereignCore.xcframework"

# deploying to ios project
echo "🚀 Deploying to iOS Project..."
mkdir -p "${CORE_DIR}/../ios/SovereignLife"
cp "${IOS_LIBS_DIR}/sovereign_core.swift" "${CORE_DIR}/../ios/SovereignLife/"

echo "✅ Build complete! Artifacts moved to ${IOS_LIBS_DIR}/ and deployed to ios/SovereignLife/"
