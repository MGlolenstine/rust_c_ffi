# Rust library in C++

## `ffi_lib`

Cargo project containing the library.

## `main.cpp`

C++ source for using the library.

## `meson.build`

Meson configuration.

## Binding generation

`cd` into the `ffi_lib` directory and execute:

```bash
cbindgen --project ffi_lib -o c/counter.hpp
```

The generated header will be placed in `ffi_lib/c/counter.hpp`.
