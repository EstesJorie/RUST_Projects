## 1. Hello, World! and Hello, Cargo!

### Hello, world!

```bash
mkdir hello_world
cd hello_world
```

Once in the Hello World directory, a simple print message can be written

```rust
fn main(){
    println!("Hello, world!")
}
```

To build and then run this program, in the terminal:

```bash 
rustc main.rs
./main
```
#### Note 
> Compliation via the Rust linker is very similar to GCC compliation in C

### Hello, Cargo!

To create a new project using Cargo, type in the terminal:

```bash
cargo new hello_cargo
cd hello_cargo
```

Build the cargo you can type:

```bash
cargo build
```

To build and then run at the same, you can type:

```bash
cargo run
```