pkgname=puff
pkgver=2.0.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="puff - a tool for managing c/c++ packages"

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" puff
}
