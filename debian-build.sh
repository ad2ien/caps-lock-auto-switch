#!/bin/bash
set -e

DEBIAN_FOLDER="debian"
BINARY_NAME="capslock-auto-switch"
VERSION="0.1.0"
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

echo "🗞️ Compress changelog..."
gzip -c -9 ./changelog > ./${DEBIAN_FOLDER}/usr/share/doc/${BINARY_NAME}/changelog.gz

echo "🖨️ Copy files..."
cp ./LICENSE ./${DEBIAN_FOLDER}/usr/share/doc/${BINARY_NAME}/copyright

echo "🧪 run tests..."
cargo test
echo "🦀 run build..."
cargo build --release
strip target/release/$BINARY_NAME
sudo cp target/release/$BINARY_NAME ${DEBIAN_FOLDER}/usr/bin

echo "🔏 Set permissions..."
SPECIAL_OWNER=$(ls -alF /${DEBIAN_FOLDER}/etc/systemd/user | grep -Ei ' ./' | awk '{print $3}')
if [ "$SPECIAL_OWNER" != "root" ]; then
    sudo chown -R root:root ./${DEBIAN_FOLDER}/usr
    sudo chown -R root:root ./${DEBIAN_FOLDER}/etc
fi
sudo chmod -R 755 ./${DEBIAN_FOLDER}/usr
sudo chmod -R 755 ./${DEBIAN_FOLDER}/etc
sudo chmod -R 755 ./${DEBIAN_FOLDER}/usr/share/
sudo chmod 644 ./${DEBIAN_FOLDER}/usr/share/doc/${BINARY_NAME}/*

echo "📦 Build Debian package..."
rm -f ${PKG_NAME}.deb
sudo dpkg-deb --build -Z xz ./${DEBIAN_FOLDER}
mv ${DEBIAN_FOLDER}.deb ${PKG_NAME}.deb

echo "🗹 check with lintian..."
set +e
docker run -it -v ./${PKG_NAME}.deb:/app/${PKG_NAME}.deb nouchka/lintian -c /app/${PKG_NAME}.deb -v

sudo chown $SPECIAL_OWNER:$SPECIAL_OWNER ./${DEBIAN_FOLDER}/etc/systemd/user
echo "🎉 Done! 🎉"