PACKAGE_NAME="capslock-auto-switch"
BINARY_NAME="capslock-auto-switch"

cargo build --release

# Create the package directory structure
mkdir -p $PACKAGE_NAME/usr/bin

cp target/release/$BINARY_NAME $PACKAGE_NAME/usr/bin

dpkg-deb --build $PACKAGE_NAME

# check with lintian
# docker run -it -v ./$PACKAGE_NAME.deb:/app/$PACKAGE_NAME.deb nouchka/lintian -c /app/$PACKAGE_NAME.deb