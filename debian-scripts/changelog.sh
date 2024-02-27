#!/bin/bash
set -e

# Needed help to get changelog format right

build_docker_image() {
    echo "build docker image..."
    docker build -t ubuntu-devscripts:latest .
}

create_changelog() {
    EMAIL=
    echo "create changelog..."
    docker run --rm -it -v $(pwd):/build -w /build \
    -e EMAIL=$EMAIL \
    ubuntu-devscripts:latest dch --create --package capslock-auto-switch --newversion 0.1.0-1 --distribution unstable "Initial release"
}

update_changelog() {
    NEW_VERSION=
    EMAIL=
    echo "update changelog..."
    mv changelog ./debian/
    docker run --rm -it -v $(pwd):/build -w /build \
    -e EMAIL=$EMAIL \
    ubuntu-devscripts:latest dch --newversion $NEW_VERSION
    mv ./debian/changelog ./
}
