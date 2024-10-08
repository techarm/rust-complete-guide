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
- Option: `Some(value)`, `None`
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
- Result: `Ok(value)`, `Err(error)`
  - `Ok()` variant is used when something went well
  - `Err()` variant used when something bad happend
  - `?` operator
- tuple

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

### Module

- Every file **and folder** makes its own separate module
- You can't do deeply nested imports
- You can to chain imports

### String, &String, &str

- String:
  ```rust
  let color = String::from("red");
  ```
  - Uses memory in: Stack and Heap
  - Use anytime we want ownership of text
  - Use anytime we want text that can grow or shrink
- &String
  ```rust
  let color = String::from("red");
  let color_ref = &color;
  ```
  - Uses memory in: Stack
  - Rarely used!
  - Rust will automatically turn &String into &str for you
- &str
  ```rust
  let color = String::from("red");
  let color_ref = color.as_str();
  ```
  - Uses memory in: Stack
  - Use anytime you don't want to take ownershop of text
  - Use anytime you want to refer to a **portion** of string owned by something else
  - Refers directly to heap-allocated or data-allocated text

#### Why is there &String and &str?

- Reason #1: `&str` lets you refer to text in the data segment without a heap allocation
- Reason #2: `&str` lets you 'slice' (take a portion) of text that is already on the heap

## Iterators

- Used to iterate over any kind of data structure
- They are used behind the scenes when you write a for loop
- Follow all the same rules of ownership, borrowing, lifetimes
- Use the Option enum

```rust
    let colors = vec![
        String::from("red"),
        String::from("yellow"),
        String::from("green"),
    ];

    let mut iter = colors.iter();
```

We usually don't call `next` on an iterator manually

- Option1: Use a for loop. Automatically creates an iterator and calls `next` on it
- Option2: Use iterator adaptors and consumers like `for_each`, `collect`, `map`, etc

### Iterator

- `iter()`: The iterator will give you a **read-only reference** to each element
- `iter_mut()`: The iterator will give you a **mutable reference** to each element
- `into_iter()`: The iterator give you **ownership** of each element, unless called on a mutable ref to a vector

### Iterator adaptor

Vec<String> -> Iterator -> adaptor (`map()`) -> consumers (`for_each()`)

### Collect

- Automatically judge based on the returned type

  ```rust
  fn to_uppercase(items: &[String]) -> Vec<String> {
    items.iter().map(|el| el.to_uppercase()).collect()
  }
  ```

- Manual setting type

  ```rust
  let result: Vec<String> = items.iter().map(|el| el.to_uppercase()).collect();
  ```

- Turbofish

  ```rust
  fn to_uppercase(items: &[String]) -> Vec<String> {
      items
          .iter()
          .map(|el| el.to_uppercase())
          .collect::<Vec<String>>()
  }
  ```

## Lifetime annotations

Help the compiler make sure refs wont outlive the value they refer to

```rust
fn next_lang<'a>(langs: &'a [String], current: &str) -> &'a str {
    let mut is_found = false;
    for lang in langs {
        if is_found {
            return lang;
        }
        if lang == current {
            is_found = true;
        }
    }
    langs.last().unwrap()
}
```

- The returned ref must refer to either the first or second argument
- Rust intentionally doesn't look at your function body to figure out if the ref is tied to the first or second argument
- Rust wants a function signature to make it clear whether the returned ref relies on the first or second argument (or both)

## Generics

- trait bound: ToPrimitive

install : `cargo add num-traits`

```rust
use num_traits::ToPrimitive;

fn main() {
    let a: i32 = 3;
    let b: f64 = 4.0;
    println!("{}", calc(a, b))
}

fn calc<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}
```

## Trait

- A trait is a set of methods
- It can contain **abstract methods** which don't have an implementation
- It can contain **default methods**, which have an implemention

```rust
trait Vehicle {
  // abstract method
  fn start(&self);

  // default method
  fn stop(&stop) {
    println!("Stopped");
  }
}
```

- A struct/enum/primitive can **implement** a trait
- The implementor has to provide an implementation for all of the **abstract methods**
- The implementor can **optionally** override the default methods

```rust
struct Car {};

impl Vehicle for Car {
  fn start(&self) {
    println("Start");
  }
}
```
