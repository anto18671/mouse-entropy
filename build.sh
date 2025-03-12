#!/bin/zsh

set -e  # Exit immediately if a command fails

PKGNAME="mouse-entropy"
PKGVER="0.1.0"
SRCTARBALL="${PKGNAME}-${PKGVER}.tar.gz"
BUILD_DIR="package_build"

echo "Cleaning up previous builds..."
rm -rf "${BUILD_DIR}" "${SRCTARBALL}"
mkdir -p "${BUILD_DIR}"

echo "Copying necessary files into ${BUILD_DIR}..."
rsync -av --exclude='.git' --exclude='.github' --exclude='target' --exclude='package_build' \
    --exclude='*.tar.gz' ./ "${BUILD_DIR}/"

# Rename the build directory inside the tarball
mv "${BUILD_DIR}" "mouse-entropy-${PKGVER}"

# Create tarball inside the repo
echo "Creating source tarball: ${SRCTARBALL}..."
tar -czf "${SRCTARBALL}" "mouse-entropy-${PKGVER}"
rm -rf "mouse-entropy-${PKGVER}"

echo "Calculating SHA256 checksums..."
SHA_TARBALL=$(sha256sum "${SRCTARBALL}" | awk '{print $1}')
SHA_SERVICE=$(sha256sum mouse-entropy.service | awk '{print $1}')
SHA_CONFIG=$(sha256sum mouse-entropy.toml | awk '{print $1}')
SHA_INSTALL=$(sha256sum mouse-entropy.install | awk '{print $1}')

echo "Updating PKGBUILD with new checksums..."
sed -i "/# --- VERIFY SOURCE ---/,/# --- VERIFY SOURCE ---/c\
# --- VERIFY SOURCE ---\n\
sha256sums=(\n\
    '${SHA_TARBALL}' \n\
    '${SHA_SERVICE}' \n\
    '${SHA_CONFIG}' \n\
    '${SHA_INSTALL}' \n\
)\n\
# --- VERIFY SOURCE ---" PKGBUILD

echo "PKGBUILD updated successfully!"
echo "============================="
echo "  Tarball: ${SRCTARBALL} - ${SHA_TARBALL}"
echo "  Service: mouse-entropy.service - ${SHA_SERVICE}"
echo "  Config:  mouse-entropy.toml - ${SHA_CONFIG}"
echo "  Install: mouse-entropy.install - ${SHA_INSTALL}"
echo "============================="
echo "Run 'makepkg -si' to test the package."
