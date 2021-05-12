# katex-rs

[![Build Status](https://github.com/xu-cheng/katex-rs/workflows/build/badge.svg)](https://github.com/xu-cheng/katex-rs/actions)
[![Latest Version](https://img.shields.io/crates/v/katex.svg)](https://crates.io/crates/katex)
[![Rust Documentation](https://docs.rs/katex/badge.svg)](https://docs.rs/katex)

This crate offers Rust bindings to [KaTeX](https://katex.org). This allows you to render LaTeX equations to HTML.

## Documentation

<https://docs.rs/katex>

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
katex = "0.4.0-alpha.1"
```

## Examples

```rust
let html = katex::render("E = mc^2").unwrap();

let opts = katex::Opts::builder().display_mode(true).build().unwrap();
let html_in_display_mode = katex::render_with_opts("E = mc^2", &opts).unwrap();
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
