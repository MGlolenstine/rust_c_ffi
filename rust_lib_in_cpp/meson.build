project('ffi_lib', 'cpp')

cpp = meson.get_compiler('cpp')

ffi_lib = cpp.find_library('ffi_lib', dirs: meson.project_source_root()+'/ffi_lib/target/debug')
threads = dependency('threads')
dl = cpp.find_library('dl', required: false)

incdir = include_directories('ffi_lib/c')

executable('main', 'main.cpp', dependencies: [ffi_lib, threads, dl], include_directories: incdir)