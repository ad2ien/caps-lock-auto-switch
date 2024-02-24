# Caps Lock Auto Switch

Use case : as any boomer, I sometimes type while looking at the keyboard. And I end up starting sentences having the case all wrong.

This small service listens to keyboard events and detects words like `hELLO `. Then the caps lock is toggled and the word is retyped.

This package is written in Rust and uses [rdev](https://docs.rs/rdev/latest/rdev/) crate. It won't work with Wayland windowing system.

> [!WARNING]  
> 🚧 Debian package under construction

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

- [ ] uninstall should remove service
- [ ] check install/remove, upgrade on a clean system
- [ ] config file for a debian package
- [ ] only define project variable once : description version..
- [ ] manage languages special characters
- [ ] debian package lint warnings
- [ ] release
- [ ] CI, lint, make package
- [ ] install instructions
- [ ] readme badges
- [ ] man page
- [ ] commands : help, edit configuration...
- [ ] dockerize build
