# Rust Mini Calculator (CLI)

A tiny interactive CLI calculator to learn **variables, types, and functions** in Rust.

## Why this project (MERN → Rust parallel)

- In MERN, you'd create a small Node CLI with `readline` and pure JS functions for math.
- Here, we do the same in Rust: standard input loop + small pure functions, compiled as a fast binary.

## Features

- Interactive REPL: enter `<op> <a> <b>` (e.g. `+ 2 3`, `mul 3 4`)
- Supported ops: `+ - * / ^` (and `add sub mul div pow`)
- Graceful errors (invalid numbers, wrong arity, division by zero)
- Unit tests (`cargo test`)

## Setup

1. Install Rust (toolchain + cargo) via rustup:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   # On Windows, use the rustup-init.exe installer
   ```

2. Check your toolchain:
   ```bash
   rustc --version
   cargo --version
   ```

3. (Recommended) VS Code + Rust Analyzer extension.

## Run

```bash
cargo run
```

Example session:
```
$ cargo run
> + 2 3
5
> mul 3 4
12
> / 10 2
5
> pow 2 3
8
> exit
Goodbye!
```

## Test
```
cargo test
```

## Build release
```
cargo build --release
# binary at target/release/rust-mini-calculator
```