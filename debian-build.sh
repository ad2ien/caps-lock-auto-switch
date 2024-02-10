PACKAGE_NAME="capslock-auto-switch"
BINARY_NAME="capslock-auto-switch"
VERSION="0.1.0"
REVISION="1"
PKG_NAME="${PACKAGE_NAME}_${VERSION}-${REVISION}_all"

SPECIAL_OWNER=$(ls -alF ./capslock-auto-switch/usr | grep -Ei ' ./' | awk '{print $3}')
if [ "$SPECIAL_OWNER" != "root" ]; then
    sudo chown -R root:root ./capslock-auto-switch/usr
    sudo chown -R root:root ./capslock-auto-switch/etc
fi

cargo build --release
strip target/release/$BINARY_NAME

# Create the package directory structure
mkdir -p $PACKAGE_NAME/usr/bin

cp target/release/$BINARY_NAME $PACKAGE_NAME/usr/bin

rm -f $PKG_NAME.deb
dpkg-deb --build -Z xz ./$PACKAGE_NAME
mv $PACKAGE_NAME.deb $PKG_NAME.deb

# check with lintian
docker run -it -v ./$PKG_NAME.deb:/app/$PKG_NAME.deb nouchka/lintian -c /app/$PKG_NAME.deb