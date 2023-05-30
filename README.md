# Rua

Rua is a tool for doing FFI code generation between Rust and other languages.
It is currently under development (not working), but we do expect it to be 
in working conditions soon.

There are three guiding philosophy for Rua:

1. Low-level: we do not want to provide much high-level abstractions. This is 
   in contrast with 
   [flutter_rust_bridge](https://docs.rs/flutter_rust_bridge/latest/flutter_rust_bridge/).
   We only want a simple layer of code generation in Rua. 
2. Simple: ideally, we want to build Rua in such a way that you do not have to 
   learn how to use it. You just simply download it and use it. This means that
   we will be providing minimal additional types. We would also try to avoid 
   the use of code generation on the Rust side as much as possible.
3. Extensible: we are currently building Rua to support for FFI between 
   [dart](https://github.com/dart-lang) and Rust. We would love Rua to be 
   designed in a way such that it could easily support other languages as well.


