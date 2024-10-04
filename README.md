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
- enum
- if, match
- Option: Some, None
  - `item.unwarp()`
    - if 'item' is a Some, returns the value in the Some
    - if 'item' is a None, panics!
    - use for quick debugging or examples
  - `item.expect("message")`
    - if 'item' is a Some, returns the value in the Some
    - if 'item' is a None, prints the provided debug message and panics!
    - Use when we want to crash if there is no value
  - `item.unwarp_or(some_object)`
    - if 'item' is a Some, reutrns the value in Some
    - if 'item' is a None, returns the provided default value
    - Use when it makes sense to provide a fallback value

### Ownership, Borrowing, Lifetimes

1. (Ownership) Every value is 'owned' by a single variable, object, argument, etc at a time
2. (Ownership) Reassinging the value to anther variable, passing it to a function, putting it into a vector, etc, `moves` the value. The old variable can't be used anymore!
3. (Borrowing) You can crate many read-only references to a value that exist at the same time
4. (Borrowing) You can't move a value while a ref to the value exists
5. (Borrowing) You can make a writeable (mutable) reference to a value only if there are no read-only references currently in use. One mutable ref to a value can exist at a time
6. (Borrowing) You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists
7. (Borrowing) Some types of values are copyied instead of moved (numbers, bools, chars, arrays/tuples with copyable elements)
8. (Lifetimes) When a variable goes out of scope, the value owned by it dropped (cleaned up in memory)
9. (Lifetimes)Values can't be dropped if there are still active references to it
10. (Lifetimes)References to a value can't outlive the value they refer to
11. These rules will dramatically change how you write code (compared to other languages)
12. When in doubt, remember that Rsut wants to minimize unexpected updates to data

#### Borrowing

- readonly reference: `&`
- mutable reference: `&mut`
  - mutable ref allow us to read or change a value without moving it
- copy-able values: numbers, bools, chars, arrays/tuples with copyable elements
