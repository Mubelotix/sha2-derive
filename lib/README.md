# sha2-derive

This crate provides a simple [`Hashable`](https://docs.rs/sha2-derive/latest/sha2-derive/trait.Hashable.html) trait that can be derived easily.
As of now, the derive macro only works on structs.

## Example

```rust
use sha2_derive::*;

#[derive(Hashable)]
struct User {
    username: String,
    age: u8,
    timestamps: Vec<u64>,
}

fn main() {
    let user = User {
        username: String::from("Mubelotix"),
        age: 42,
        timestamps: vec![1, 2, 3],
    };

    let hash = user.hash();
}
```

## Implementing Hashable on a custom type

```rust
struct CustomType {
   // fields
}

impl Hashable for CustomType {
    fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
        todo!()
    }
}
```

License: MIT
