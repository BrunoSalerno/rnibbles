Rnibbles
=======
Nibbles game in Rust using Bevy framework.

Cross compilation
-----------------
1. Install [Cross](https://github.com/cross-rs/cross) and [cross-toolchains](https://github.com/cross-rs/cross-toolchains).
2. Build cross image for the desired arch (in this case, msvc x86_64)
```
cargo build-docker-image x86_64-pc-windows-msvc-cross
```
3. Make sure that the [Cross.toml fine](Cross.toml) has an entry for your desired arch, pointing to the image just built.
Note. Some archs already have default images.

4. Build the release`using the right arch:
```
cross build --release --target x86_64-pc-windows-msvc
```
