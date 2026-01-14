# C++ Rust Compiler

A C++ compiler written in Rust.

## Project Structure

This workspace contains several crates that handle different aspects of the compilation process:

- `lexer`: Tokenizes C++ source code
- `parser`: Parses tokens into an abstract syntax tree
- `ast`: Defines the abstract syntax tree data structures
- `semantic`: Performs semantic analysis and type checking
- `types`: Implements the type system
- `ir`: Contains the intermediate representation
- `codegen`: Generates Rust code from the IR
- `driver`: Main executable that orchestrates the compilation process
- `utils`: Common utility functions
- `tests`: Integration and unit tests

## Building

To build the project:

```bash
cd c2rust
cargo build
```

## Running

To run the compiler:

```bash
cargo run --bin c2rust [options] input.cpp
```

## License

MIT
