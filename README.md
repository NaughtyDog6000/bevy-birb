# BevyBirb

Creating Flappy Bird Clone in bevy for both wasm and native platforms

wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "bevy_birb" target/wasm32-unknown-unknown/release/bevy_birb.wasm

wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "bevy_birb" target/wasm32-unknown-unknown/debug/bevy_birb.wasm

cargo build --release --target wasm32-unknown-unknown

cargo watch -cx run

cargo watch -cx "run --target wasm32-unknown-unknown"

_Moved over to trello_

## bugs

Investigate audio on wasm and how it causes fps issues

## Default Controls

Space - Flap
R - Restart
Escape - Quit
M - Toggle Audio Playback
F11 - Toggle Fullscreen


## ignore

cd Documents/files-to-serve-from-caddy/bevybirb