SHA=$(shasum -a 256 "${BASH_SOURCE%/*}/app/epick.app.tar.gz")
SHA=$(SHA=$SHA swift parse_sha.swift)

CASK=$(VERSION=$VERSION SHA=$SHA URL=$URL swift build_cask.swift)
printf "%s" "$CASK"