Rnibbles
=======
3D Nibbles game in Rust using the Bevy framework.

Building and running
--------------------

Make sure Rust and Cargo are installed.

1. Clone this repo
2. Run `cargo run`

Cross compilation
-----------------
1. Install [Cross](https://github.com/cross-rs/cross) and [cross-toolchains](https://github.com/cross-rs/cross-toolchains).
2. Build cross image for the desired arch (in this case, msvc x86_64)
```bash
cargo build-docker-image x86_64-pc-windows-msvc-cross
```
3. Make sure that the [Cross.toml file](Cross.toml) has an entry for your desired arch, pointing to the image just built.
Note. Some archs already have default images.

4. Build the release`using the right arch:
```bash
cross build --release --target x86_64-pc-windows-msvc
```
