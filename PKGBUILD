# Maintainer: Anthony Therrien
pkgname=mouse-entropy
pkgver=0.1.0
pkgrel=1
pkgdesc="A tool to capture mouse data (requires root)."
arch=('x86_64')
url="https://github.com/anto18671/mouse-entropy"
license=('MIT')
depends=('rust')
makedepends=('cargo')
source=(
    "mouse-entropy-${pkgver}.tar.gz"
    "mouse-entropy.service"
    "mouse-entropy.toml"
    "mouse-entropy.install"
)

# --- VERIFY SOURCE ---
sha256sums=(
    '43e380ed9317fa58759e76a5b7151e755aa955b3774642d1f59dc90ac00e32c8' 
    'e8f2ed265f90f70d32dfd0ee0b41e6960e5dffdbd8e3765d4b1e5c597d8e9430' 
    '3bf6348e06deffc864f257e0f7b301b2b3105f462741e5b494937abefc2d4a0e' 
    '538cc0e9ec1bfa75fd0880932c8972f68ae8fe2a92958c44e5760dcc4b569c51' 
)
# --- VERIFY SOURCE ---

prepare() {
  cd "$srcdir/mouse-entropy-${pkgver}"
  cargo fetch
}

build() {
  cd "$srcdir/mouse-entropy-${pkgver}"
  cargo build --release --locked
}

package() {
  cd "$srcdir/mouse-entropy-${pkgver}"

  # Install the binary
  install -Dm755 "target/release/mouse-entropy" "$pkgdir/usr/bin/mouse-entropy"

  # Install the default config
  install -Dm644 "$srcdir/mouse-entropy.toml" "$pkgdir/etc/mouse-entropy/mouse-entropy.toml"

  # Install the systemd service
  install -Dm644 "$srcdir/mouse-entropy.service" "$pkgdir/usr/lib/systemd/system/mouse-entropy.service"
}

# Post-install hooks
install="mouse-entropy.install"
