#!/bin/bash
set -e

# Directory setup
cd "$(dirname "$0")"
CORE_DIR=$(pwd)
IOS_DIR="../ios"
BUILD_DIR="target"
BINDINGS_DIR="${IOS_DIR}/SovereignBindings"

echo "🚀 Starting iOS Build Process..."

# Ensure cargo-lipo is installed
if ! command -v cargo-lipo &> /dev/null; then
    echo "📦 Installing cargo-lipo..."
    cargo install cargo-lipo --locked
fi

# Build static library for iOS targets
echo "🛠 Building Rust library for iOS..."
cargo lipo --release

# Generate Swift bindings using uniffi-bindgen
# Note: Assuming uniffi-bindgen is installed or available via cargo run features
# If uniffi-bindgen CLI is not installed, we can try running it from the dependency if configured,
# but usually it's better to expect the user to have it or install it.
# Here we'll install it if missing for robustness, matching the library version.
if ! command -v uniffi-bindgen &> /dev/null; then
    echo "📦 Installing uniffi-bindgen..."
    cargo install uniffi --version 0.31 --features cli --locked
fi

echo "📝 Generating Swift bindings..."
# We need to find the dylib or rlib to generate bindings from, usually in target/release or target/debug/deps
# For uniffi, we typically pass the library path.
# However, uniffi-bindgen generate takes the UDL or the library if using proc-macros.
# Assuming proc-macros (modern uniffi), we point to the dylib/cdylib.
# cargo lipo --release puts things in target/universal/release, but uniffi needs the structural info.
# We often build a native lib for bindings generation first if cross-compiling is complex.
cargo build --release
LIBRARY_PATH="${BUILD_DIR}/release/libcore.dylib" 
# MacOS uses .dylib, Linux .so. uniffi-bindgen needs to load it.

if [ ! -f "$LIBRARY_PATH" ]; then
    echo "❌ Could not find library at $LIBRARY_PATH for binding generation."
    exit 1
fi

mkdir -p "${BINDINGS_DIR}"
uniffi-bindgen generate "${LIBRARY_PATH}" --language swift --out-dir "${BINDINGS_DIR}" --no-format

# Create XCFramework
echo "📦 Creating XCFramework..."
UNIVERSAL_LIB="${BUILD_DIR}/universal/release/libcore.a"

if [ ! -f "$UNIVERSAL_LIB" ]; then
    echo "❌ Could not find universal library at $UNIVERSAL_LIB"
    exit 1
fi

# Clean previous xcframework
rm -rf "${IOS_DIR}/SovereignCore.xcframework"

# Create the xcframework
xcodebuild -create-xcframework \
    -library "${UNIVERSAL_LIB}" \
    -headers "${BINDINGS_DIR}" \
    -output "${IOS_DIR}/SovereignCore.xcframework"

echo "✅ Build complete! Artifacts moved to ${IOS_DIR}/SovereignCore.xcframework"
