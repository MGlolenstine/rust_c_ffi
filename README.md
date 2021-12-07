# Rust FFI example

- [C++ library with Rust](https://github.com/MGlolenstine/rust_c_ffi/tree/main/cpp_lib_in_rust)
- [Rust library with C++](https://github.com/MGlolenstine/rust_c_ffi/tree/main/rust_lib_in_cpp)

## C++ library with Rust

### Library compilation

```bash
gcc -g -c -o native.o wrapper.c
```

|Option|Meaning|
|---|---|
|`-g`|Debug information (enables debug of C files)|
|`-c`|Don't link, it's a library|
|`-o`|Output compiled result as `native.o`|

### Library to static library

```bash
ar rsc libnative.a native.o
```

- archives `native.o` into a static library archive `libnative.a`

### Bindings are generated automatically

## Rust library with C++

`Cargo.toml` must contain the following so it can produce a static library.

```toml
[lib]
name = "ffi_lib"
path = "src/lib.rs"
crate-type = ["staticlib"]
```

### Generate bindings

```bash
cbindgen --project ffi_lib -o c/counter.hpp
```

### Manual Compilation

```bash
g++ main.cpp -ldl -lpthreads -Lffi_lib/target/debug -lffi_lib -o main
```

|Option|Meaning|
|---|---|
|`-ldl`| include `dl`|
|`-lpthreads`|include `pthreads`|
|`-Lffi_lib/target/debug`|add a new search location for `ffi_lib`|
|`-lffi_lib`|include `ffi_lib`|

### Automatic Compilation

```bash
meson setup builddir
cd builddir
ninja
```
