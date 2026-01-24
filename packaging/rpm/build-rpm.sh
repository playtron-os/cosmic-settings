#!/bin/bash
# Build RPM for cosmic-settings
# Usage: ./packaging/rpm/build-rpm.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "$PROJECT_DIR"

BINARY="cosmic-settings"
VERSION=$(grep '^version' cosmic-settings/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}"
RPM_ROOT="${CARGO_TARGET_DIR}/rpm-root"
RPM_BUILD="${CARGO_TARGET_DIR}/rpm-build"

echo "Building ${BINARY} version ${VERSION}..."

# Build release binary
cargo build --release -p cosmic-settings

echo "Building RPM..."
rm -rf "${RPM_ROOT}" "${RPM_BUILD}"
mkdir -p "${RPM_ROOT}" "${RPM_BUILD}"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Build RPM with source directory and version passed via environment
COSMIC_SETTINGS_SOURCE="${PROJECT_DIR}" COSMIC_SETTINGS_VERSION="${VERSION}" rpmbuild -bb --nodeps \
    --define "_topdir ${PROJECT_DIR}/${RPM_BUILD}" \
    --define "_binary_payload w2.xzdio" \
    --buildroot "${PROJECT_DIR}/${RPM_ROOT}" \
    packaging/rpm/cosmic-settings.spec

# Copy to dist folder
mkdir -p dist
cp -v "${RPM_BUILD}"/RPMS/*/*.rpm dist/

echo "RPM created in dist/"
ls -la dist/*.rpm
