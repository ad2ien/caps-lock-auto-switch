#!/bin/bash
set -e

PACKAGE_NAME="capslock-auto-switch"
BINARY_NAME="capslock-auto-switch"
VERSION="0.1.0"
REVISION="1"
PKG_NAME="${PACKAGE_NAME}_${VERSION}-${REVISION}_all"

echo "🏗️ Start building debian package..."

echo "🗑️ cleaning..."
sudo rm -rf ${PACKAGE_NAME}/usr
sudo rm -rf ${PACKAGE_NAME}.deb

echo "🛠️ Build package structure"
mkdir -p ${PACKAGE_NAME}/etc/systemd/user
mkdir -p ${PACKAGE_NAME}/usr/share/doc/${PACKAGE_NAME}
mkdir -p ${PACKAGE_NAME}/usr/bin

echo "🗞️ Compress changelog..."
gzip -c -9 ./changelog > ./capslock-auto-switch/usr/share/doc/capslock-auto-switch/changelog.gz

echo "🖨️ Copy files..."
cp ./LICENSE ./${PACKAGE_NAME}/usr/share/doc/${PACKAGE_NAME}/copyright
# cp ./capslock-auto-switch.service ./capslock-auto-switch/etc/systemd/user/capslock-auto-switch.service

echo "🧪 run tests..."
cargo test
echo "🦀 run build..."
cargo build --release
strip target/release/$BINARY_NAME
sudo cp target/release/$BINARY_NAME $PACKAGE_NAME/usr/bin

echo "🔏 Set permissions..."
SPECIAL_OWNER=$(ls -alF /capslock-auto-switch/etc/systemd/user | grep -Ei ' ./' | awk '{print $3}')
if [ "$SPECIAL_OWNER" != "root" ]; then
    sudo chown -R root:root ./capslock-auto-switch/usr
    sudo chown -R root:root ./capslock-auto-switch/etc
fi
sudo chmod -R 755 $PACKAGE_NAME/usr
sudo chmod -R 755 $PACKAGE_NAME/etc
sudo chmod -R 755 $PACKAGE_NAME/usr/share/
sudo chmod 644 $PACKAGE_NAME/usr/share/doc/$PACKAGE_NAME/*

echo "📦 Build Debian package..."
rm -f $PKG_NAME.deb
sudo dpkg-deb --build -Z xz ./$PACKAGE_NAME
mv $PACKAGE_NAME.deb $PKG_NAME.deb

echo "🗹 check with lintian..."
set +e
docker run -it -v ./$PKG_NAME.deb:/app/$PKG_NAME.deb nouchka/lintian -c /app/$PKG_NAME.deb -v

sudo chown $SPECIAL_OWNER:$SPECIAL_OWNER ./capslock-auto-switch/etc/systemd/user
echo "🎉 Done! 🎉"