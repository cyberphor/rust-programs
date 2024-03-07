# rust-programs
Programs written in Rust.

## Notes
**Step 1.** Install Rust. The Rust installer will install `rustup`, `rustc`, and `cargo` among other things.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Step 2.** Create a new Rust project (e.g., called `demo`).
```bash
cargo new demo
```

**Step 3.** Add code.

**Step 4.** Compile your source code into a program. The command below specifies the path to the `Cargo.toml` file (this is necessary if you are not within the same folder as it). After your code compiles, there will be a `Cargo.lock` file under `src/` and a binary under `target/debug/`. The binary file is your program.
```bash
cargo build --manifest-path demo/Cargo.toml
```

**Step 5.** Run your program. Again, the binary (called `demo`) is located under `target/debug/` within the `demo/` project directory. 
```bash
./demo/target/debug/demo
```

## Copyright
This project is licensed under the terms of the [MIT license](/LICENSE). 