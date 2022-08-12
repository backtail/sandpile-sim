# Sandpile Sim
Implements the sandpile algorithm and prints the output into the console! (for now)

### Install Rust
To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions. See ["Other Installation Methods"](https://forge.rust-lang.org/infra/other-installation-methods.html) if you are on Windows.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
``` 
### Build and Compile
`git clone` this repository then `cargo run --release` on main branch for optimal performance

### How to use
- choose a `num_grains` (in release mode, everything under 10000 will be calculated under a second)
- choose a `side_length` of the display square (experiment a little, 75 works for 10000 grains)
- run the program in release mode and enjoy the patterns