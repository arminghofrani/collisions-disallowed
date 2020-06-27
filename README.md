# What is this?

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/arminghofrani/collisions-disallowed/blob/master/LICENSE)

This is a cool trajectory finder, rewritten in Rust from [johnBuffer/NoCol](https://github.com/johnBuffer/NoCol).

## Compilation

Install the dependencies for [ggez](https://github.com/ggez/ggez):
```
sudo apt install libasound2-dev libudev-dev pkg-config
```
Then:
```
cargo run --release
```
If you run into problems with Wayland, you might need to:
```
export WINIT_UNIX_BACKEND=x11
```

## Physics

Some notes:

* The circles are attracted to the center, not to each other.
* The sizes of the circles don't affect their orbital period.
