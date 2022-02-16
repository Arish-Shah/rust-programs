# programming_concepts

## Variables and Mutability

1. In order to reassign variables, use `mut` keyword.

```rust
let x = 5;
x = 6; // throws exception

let mut x = 5; // works
```
2. Reassigning variables:

```rust
let mut spaces = "   ";
spaces = spaces.len(); // ERROR: string -> integer

let spaces = "  ";
let spaces = spaces.len(); // ✔
```

