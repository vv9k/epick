#!/bin/bash

# Build release version (NOTE: I am not an expert on rust's targets)
echo "Building..."
cargo build --release --target x86_64-appple-darwin

# Copy build to app
cp "${BASH_SOURCE%/*}/../../target/x86_64-apple-darwin/release/epick" "${BASH_SOURCE%/*}/app/epick.app/Contents/Resources/epick"

# Generate icons
echo "Generating icons..."
"${BASH_SOURCE%/*}/../../assets/app-icon-macos/icon/generate_icons.sh"
# Copy icons
cp "${BASH_SOURCE%/*}/../../assets/app-icon-macos/icon/AppIcon.icns" "${BASH_SOURCE%/*}/app/epick.app/Contents/Resources/AppIcon.icns"