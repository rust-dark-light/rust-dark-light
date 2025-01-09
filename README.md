<div align="center">
    <img src="resources/icon/icon.svg" width="200"/>
    <h1>dark-light</h1>
    <p>Rust crate to detect the system theme mode</p>
    <a href="https://matrix.to/#/#dark-light:matrix.org"><img alt="Matrix" src="https://img.shields.io/matrix/dark-light%3Amatrix.org?style=for-the-badge"></a>
    <br>
    <br>
</div>

Supports macOS, Windows, Linux, BSDs, and WebAssembly.

On Linux the XDG Desktop Portal D-Bus API is checked for the `color-scheme` preference, which works in Flatpak sandboxes without needing filesystem access.

[API Documentation](https://docs.rs/dark-light/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dark-light = "2.0.0"
```


### Detect current theme mode
You can detect the current mode by using the `detect` function. This function returns a `Mode` value.
```rust
fn main() -> Result<(), dark_light::Error> {
    match dark_light::detect()? {
        dark_light::Mode::Dark => println!("Dark mode"),
        dark_light::Mode::Light => println!("Light mode"),
        dark_light::Mode::Unspecified => println!("Unspecified"),
    }
    Ok(())
}
```

## License

Licensed under either of the following licenses:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
