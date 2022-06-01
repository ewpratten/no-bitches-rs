# NO BITCHES?
[![Crates.io](https://img.shields.io/crates/v/no-bitches)](https://crates.io/crates/no-bitches)
[![Docs.rs](https://docs.rs/no-bitches/badge.svg)](https://docs.rs/no-bitches)
[![Build](https://github.com/Ewpratten/no-bitches-rs/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/no-bitches-rs/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/no-bitches-rs/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/no-bitches-rs/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/no-bitches-rs/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/no-bitches-rs/actions/workflows/audit.yml)

Continuing on the meme theme of my last few projects.. A Rust library that generates [megamind memes](https://knowyourmeme.com/memes/no-bitches). Feel free to find some horrible way to integrate this into your own project. I designed it for use in a personal Discord bot.

## Using

```rust
use no_bitches::build_megamind_meme;

let meme = build_megamind_meme("No Unsafe Code?", None);
meme.save("./meme.png").unwrap();
```
