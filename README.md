# rust-dll-example

Here is a simple example of how to compile a Rust library into a DLL.
It includes plain functions, fixed-length and pointer-based arrays, structures
and OOP style. The check is done with Python in the file `check_dll.py`.


## How to create a DLL project in Rust

1. Create a project as a library: `cargo new rust-dll-example --lib`

2. Add the following section in `Cargo.toml`:

```
[lib]
crate-type = ["cdylib"]
```

3. Add `#[no_mangle]` and `extern` to the exporting functions like this:

```rust
#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}
```

4. Build the project: `cargo build --release`

After the the DLL will appear in `./target/release/rust_dll_example.dll`.
