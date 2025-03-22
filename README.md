# gui-playground

This is a collection of a few toy applications I'm building for fun that all have a GUI intensive part of the coding and
share the same macroquad abstractions.

Hosted on https://fridge-dev.github.io/gui-playground

## Apps

* [Caterpillar](./caterpillar) - Modified snake example
* [Turn Time Tracker](./turn-time-tracker) - Tool to track tabletop game time per player
* [Mastermind](./mastermind) - Classic tabletop game built from scratch

# Misc Learning Docs

Main learning take-away: Macroquad is good for drawing, bevy is good for ECS. I'm just doing some drawing.

* https://macroquad.rs/examples/
* https://www.reddit.com/r/rust_gamedev/comments/oz5kd9/macroquad_vs_bevy/
* https://bevyengine.org/learn/quick-start/getting-started/ecs/

## WASM Support

https://github.com/not-fl3/macroquad#wasm

One time setup for local development:

```
rustup target add wasm32-unknown-unknown
cargo install basic-http-server
```

Build

```
./build-check.sh
./wasm-build.sh
```

Run web server

```
# start
./run-web-server.sh

# kill
./kill-web-server.sh
```
