# Caps Lock Auto Switch

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ad2ien/caps-lock-auto-switch/release.yml?label=release&logo=github&branch=main)
![License](https://img.shields.io/badge/license-GPL%202%2B%202.0-blue.svg)
![Gitmoji](https://img.shields.io/badge/gitmoji-%20%F0%9F%98%9C%20%F0%9F%98%8D-FFDD67.svg)

Use case : as any boomer, I sometimes type while looking at the keyboard. And I end up starting sentences having the case all wrong.

This small service listens to keyboard events and detects words like `hELLO `. Then the caps lock is toggled and the word is retyped.

This package is written in Rust and uses [rdev](https://docs.rs/rdev/latest/rdev/) crate. It won't work with Wayland windowing system.

> [!WARNING]  
> 🚧 Debian package under construction

## Install

Run the following commands to install the package:

```bash
VERSION=$(curl "https://api.github.com/repos/ad2ien/caps-lock-auto-switch/tags" | jq -r '.[0].name')
curl -L "https://github.com/ad2ien/caps-lock-auto-switch/releases/download/${VERSION}/capslock-auto-switch_${VERSION#v}-1_all.deb -o capslock-auto-switch_${VERSION#v}-1_all.deb"
sudo dpkg -i capslock-auto-switch_${VERSION#v}-1_all.deb
rm capslock-auto-switch_${VERSION#v}-1_all.deb
```

## Dev

Pre requisites:

```bash
sudo apt-get install libx11-dev xorg-dev libxdo-dev
```

Run:

```bash
cargo run
```

## Build debian package

```bash
./debian-build.sh --lint 
```

Off course lint arg is optional

## Test

Only tried on Pop!_OS 22.04 LTS. Should work on any Debian based distri with X11.

## TODO

- [ ] debian package lint warnings
- [ ] remove test from build script
- [ ] readme badges rust & lint
- [ ] commands : help, edit configuration...
- [ ] check install/remove, upgrade on a clean system
- [ ] only define project variable once : description licence version..
- [ ] manage languages special characters
- [ ] dockerize build
- [ ] logs
- [ ] docker image project for dch
- [ ] Have a debian repository to enable `apt-get install capslock-auto-switch`
