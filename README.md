# Unity-Rust
An example of using Rust in Unity
***
**Note**: This is a extremely basic example, obviously you can do a lot more with Rust and Unity other than just a basic RNG.
## Building the Rust DLL
1. Clone the repository
2. CD into the `Rust` directory
3. Build the DLL using `cargo build --release`
4. Done, the DLL is inside `target/release`
### DLL Import name
The name to use when using `DllImport` is `UnityRust`, which can be changed in `Rust/Cargo.toml`
