# What is this?

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/arminghofrani/collisions-disallowed/blob/master/LICENSE)

This is a cool trajectory finder, rewritten in Rust from [johnBuffer/NoCol](https://github.com/johnBuffer/NoCol). The program randomly initialises the position and velocity vectors of circles, according to parameters you specify. The circles are all attracted to the center. Through colliding with each other, they eventually find 'stable' orbits without collisions.

## Demo

![Fast demo](https://github.com/arminghofrani/collisions-disallowed/blob/master/demo/fast.gif)

*Parameters: n_circles = 10, max_radius = 40, max_velocity = 250, attraction_factor = 0.05, (sped up)*

![Slow demo](https://github.com/arminghofrani/collisions-disallowed/blob/master/demo/slow.gif)

*Parameters: n_circles = 25, max_radius = 40, max_velocity = 250, attraction_factor = 0.05, (slowed down, zoomed in)*

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
* The size of a circle doesn't affect its orbital period.
