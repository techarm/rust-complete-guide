# rust-complete-guide

Understand ownership, lifetimes, traits, generics, and much more through practical projects

## Baisc

- `struct`
- `#[derive(Debug)]`
- `{:?}`, `{:#?}`
- arrays, vectors
- immutable, mutable: use the `mut` keyword
- impl struct (add a function to struct)
  - associated functions (Class method)
  - method, operates on a specific instance of a struct
- implicit `return`
- crate == package
  - rust standard library
    - included with every project without any additional install
    - docs at: **doc.rust-lang.org/std**
  - external creates
    - have to be installed into our project with `cargo add <name>`
    - create listing at: **crates.io**
    - doc also at: **docs.rs**
- use external modules use the `crate` keyword
- use internal modules use the `mod` keyword
