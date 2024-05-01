#!/bin/bash
set -e

DEBIAN_FOLDER="debian"
BINARY_NAME="capslock-auto-switch"
VERSION="0.2.0"
REVISION="1"
PKG_NAME="${BINARY_NAME}_${VERSION}-${REVISION}_all"

echo "🏗️ Start building debian package..."

echo "🗑️ cleaning..."
sudo rm -rf ${DEBIAN_FOLDER}/usr
rm -rf ${BINARY_NAME}*.deb

echo "🛠️ Build package structure"
mkdir -p ${DEBIAN_FOLDER}/etc/systemd/user
mkdir -p ${DEBIAN_FOLDER}/usr/share/doc/${BINARY_NAME}
mkdir -p ${DEBIAN_FOLDER}/usr/bin
mkdir -p ${DEBIAN_FOLDER}/usr/share/man/man1

echo "🗞️ Compress changelog..."
gzip -c -n -9 ./changelog > ./${DEBIAN_FOLDER}/usr/share/doc/${BINARY_NAME}/changelog.gz

echo "🖨️ Copy files..."
cp ./LICENSE ./${DEBIAN_FOLDER}/usr/share/doc/${BINARY_NAME}/copyright

echo "📝 Make man..."
# docker run --rm -v $(pwd):/working -w /working pandoc/minimal:3 --to man man.md -o capslock-auto-switch.1
gzip -c -n -9 ./debian-scripts/man.1 > ./${DEBIAN_FOLDER}/usr/share/man/man1/capslock-auto-switch.1.gz

echo "🦀 run build..."
cargo build --release
strip target/release/$BINARY_NAME
sudo cp target/release/$BINARY_NAME ${DEBIAN_FOLDER}/usr/bin

echo "🔏 Set permissions..."
SPECIAL_OWNER=$(ls -alF ./README.md | grep -Ei ' ./' | awk '{print $3}')
if [ "$SPECIAL_OWNER" != "root" ]; then
    sudo chown -R root:root ./${DEBIAN_FOLDER}/usr
    sudo chown -R root:root ./${DEBIAN_FOLDER}/etc
fi
sudo chmod -R 755 ./${DEBIAN_FOLDER}/usr
sudo chmod -R 755 ./${DEBIAN_FOLDER}/etc
sudo chmod -x ./${DEBIAN_FOLDER}/etc/systemd/user/capslock-auto-switch.service
sudo chmod -R 755 ./${DEBIAN_FOLDER}/usr/share/
sudo chmod 644 ./${DEBIAN_FOLDER}/usr/share/doc/${BINARY_NAME}/*
sudo chmod a-x ./${DEBIAN_FOLDER}/usr/share/man/man1/capslock-auto-switch.1.gz

echo "📦 Build Debian package..."
rm -f ${PKG_NAME}.deb
sudo dpkg-deb --build -Z xz ./${DEBIAN_FOLDER}
mv ${DEBIAN_FOLDER}.deb ${PKG_NAME}.deb

if [ "$1" = "--lint" ]; then
    echo "🗹 check with lintian..."
    set +e
    docker run --rm -it -v ./${PKG_NAME}.deb:/app/${PKG_NAME}.deb nouchka/lintian -c /app/${PKG_NAME}.deb -v
fi

sudo chown -R $SPECIAL_OWNER:$SPECIAL_OWNER ./${DEBIAN_FOLDER}/etc
sudo chown -R $SPECIAL_OWNER:$SPECIAL_OWNER ./${DEBIAN_FOLDER}/usr

echo "🎉 Done! 🎉"