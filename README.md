# turn-time-tracker

A helpful tool for finding who's turn is taking the most time when playing tabletop games together.

# Usage

Currently, player names, color, and ordering must be done in `main.rs`, and then compile and start the app via `cargo run`.

Maybe I'll eventually get around to implementing UI to select players at runtime, but for now this does what I need.

## Controls

* **spacebar** - next player's turn
* **p** - pause/unpause
* **d** - detailed mode toggle

# Examples

![running](./readme-assets/app-running.png)

![paused](./readme-assets/app-paused.png)

# Misc Learning Docs

Main learning take-away: Macroquad is good for drawing, bevy is good for ECS. I'm just doing some 

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

Build (and rebuild)

```
cargo build --target wasm32-unknown-unknown
```

Run web server (learning opportunity: replace with systemd)

```
# start
basic-http-server web &

# kill
kill -9 $(ps -ef | grep "basic-http-server web" | grep -v "grep" | awk '{print $2}')
```
