# Unity-Rust
An example of using Rust in Unity
***
**Note**: This is an extremely basic example, obviously you can do a lot more with Rust and Unity other than just a basic RNG.
## Building the Rust DLL
1. Clone the repository
2. CD into the `Rust` directory
3. Build the DLL using `cargo build --release`
4. Done, the DLL is inside `target/release`
### DLL Import name
The name to use when using `DllImport` is `UnityRust`, which can be changed in `Rust/Cargo.toml`
***
> **Warning**:
> Passing strings from Rust to C# isn't as simple as just returning a `String`, etc.
>
> Here's an example of what passing strings in Rust to C# looks like:
```rust
#[no_mangle]
pub extern "C" fn string_passing() -> *mut c_char {
    let res = CString::new("Hello from Rust, issue #128!").expect("CString::new() failed!\n");
    res.into_raw() // This is returned as an IntPtr on the C#-side
}
```
> Then as you can see from the comment, it's returned as an `IntPtr` which you can read using `Marshal`.
> Note: Tagging your `.dll` with **Load plugin on Startup** is recommended as it can help avoid DLL not found exceptions.
