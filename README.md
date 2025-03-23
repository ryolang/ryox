![Ryo](./docs/assets/ryo.svg)

A statically typed programming language with a focus on simplicity and ease of use.

[⚡ Ryo](https://ryolang.org)

## Features

Primary goal: Efficient and safe Python

review https://docs.modular.com/mojo/manual/values/ownership copy design

- Expressive, Minimal Syntax for readability similar to Python
- Static Type System with strong compile-time checks
- Type Inference to minimize explicit annotations
- Memory Safety through compile-time guarantees
- Immutable-by-Default variables for safer code
- Efficient Compilation targeting native performance
- Simple async calls avoiding coloring
- Interactive REPL for rapid prototyping
- Python interoperability

## Target audiences

- Backend Services and Microservices Developers
- Finance/Fintech Application Developers
- Scientific and Technical Computing
- Data Scientists
- Web Developers
- General Purpose Scripting/Automation (Critical Infrastructure)

## Quick Start

Read the [Quick Start Guide](https://ryolang.org) from the documentation.

## Building from Source

Ensure you have [Rust](https://rust-lang.org) installed.

```shell
cargo build
```

## TODO

- Move index to getting started
- Review Memory
- Review Concurrency
### Core language
- Lexer: Lexical, Syntactic Grammar
  1. Arithmetic
  2. Basic types
  3. Operators
  4. Comments
  5. print
  6. Control Flow: if, for
  7. Functions
  8. Variables
  9. Imports
  10. Enums
      1. match
  11. Errors
      1. try..or
      2. check ? for error propagation
  12. Structs
  13. Traits
- Parser
- AST
- Interpreter (AST visitor)
- REPL
- Basic Compiler
- Basic Mem Management
- Testing - go to: lexer next number
### Safety & Concurrency
- Borrow & Ownership
- async, await, fibers
- STD
- Testing
  

## Resources

- https://www.python-httpx.org/
- https://mlir.llvm.org/docs/Dialects/
- https://edgl.dev/blog/mlir-with-rust/
- https://github.com/edg-l/edlang
- https://github.com/ehsanmok/create-your-own-lang-with-rust/tree/master
- https://createlang.rs
- https://pest.rs/book/
- https://doc.rust-lang.org/rust-by-example
- https://buzz-lang.dev/guide/
- https://github.com/rust-lang/rustlings/
- https://github.com/zesterer/tao
- https://course.ccs.neu.edu/cs4410sp19/lec_type-inference_notes.html
- https://rustc-dev-guide.rust-lang.org/type-inference.html
- https://github.com/rhaiscript/rhai

```shell
cargo run "1+1"
```