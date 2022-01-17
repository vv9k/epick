#!/bin/bash

# Build release version (NOTE: I am not an expert on rust's targets)
echo "Building..."
# cargo build --release --target x86_64-apple-darwin

# Copy build to app
cp "${BASH_SOURCE%/*}/../../target/x86_64-apple-darwin/release/epick" "${BASH_SOURCE%/*}/app/epick.app/Contents/Resources/epick"

# Generate icons (Only has to be done when the icon is updated + don't forget to also change the icon of the epick.app file)
# echo "Generating icons..."
# "${BASH_SOURCE%/*}/../../assets/app-icon-macos/icon/generate_icons.sh"
# Copy icons
# cp "${BASH_SOURCE%/*}/../../assets/app-icon-macos/icon/AppIcon.icns" "${BASH_SOURCE%/*}/app/epick.app/Contents/Resources/AppIcon.icns"

# Generate archive
echo "Bundling..."
tar -czf "${BASH_SOURCE%/*}/app/epick.app.tar.gz" "${BASH_SOURCE%/*}/app/epick.app"

# Build Cask
