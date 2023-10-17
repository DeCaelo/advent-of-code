workspace: https://www.youtube.com/watch?v=QZKWEEO-Uoo

```
cargo new --lib --vcs none day02
```

```
cargo new --vcs none day01
```

- https://www.youtube.com/watch?v=S3c7NRS698A

```
cargo build -p day01
```

```
rustup update -- nightly
rustup default nightly
```

```rust
fn main() {
  let foo = 3;
  println!("{foo:#?}");
}
```

#### Note:

```
:#? is pretty-printed Debug output
:? is normal Debug output
no modifier is Display output
```

[Display](https://doc.rust-lang.org/stable/std/fmt/trait.Display.html) is for user-facing output

[Debug](https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html) is for output when debugging, also used for panic messages
