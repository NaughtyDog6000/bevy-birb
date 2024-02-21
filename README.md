# BevyBirb

Creating Flappy Bird Clone in bevy for both wasm and native platforms

wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "bevy_birb" target/wasm32-unknown-unknown/release/bevy_birb.wasm

cargo watch -cx run

cargo watch -cx "run --target wasm32-unknown-unknown"

## TODO

- lose on out of bounds
- lose on collision with pipe
- rotation based upon y velocity
- sprite / img
- Proc gen of pipes
- score
- upload to leaderboard / signin
