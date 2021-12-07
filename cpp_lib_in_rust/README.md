# C++ library in Rust

## `nativelib-sys`

Crate used for low-level bindings.
The `src/build.rs` automatically generates rust bindings from the source code present in `c/wrapper.h` and includes it in `src/lib.rs`.

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

## `nativelib`

Safe wrapper crate around the low-level bindings.
It creates functions that verify and call functions in `nativelib-sys`.

## `consumer`

An example of end-user of bindings.
It uses safe wrapper `nativelib`.
