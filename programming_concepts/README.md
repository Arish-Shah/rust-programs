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
## Data Types

### Primitive Types

#### 1. Integer Types:

- Signed integers (contains negative): `i8` (8-bit) through `i128` (128-bit).
- Unsigned integers (only positive integers): `u8` (8-bit) through `u128` (128-bit).

Assigning largers numbers result in *integer overflow* error.

```rust
let x: i32 = 20;

```

#### 2. Floating-Point Types:

Numbers with decimal points. Rust has two primitives: `f32` and `f64` (default as it is roughly the same speed as `f32`).

```rust
let x: f64 = 20.9;

```

#### 3. Boolean Type:

Booleans are 1 byte in size and can hold two values: `true` and `false`.

```rust
let t = true;
let f: bool = false;
```

#### 4. Character Type:

`char` literal is specified in single quotes. Four bytes in size.

```rust
let z = 'Z';
```

### Compound Types

#### 1. Tuple

Tuple is a general way of grouping different types into one compount type. *Tuples have fixed size*.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Accessing values from a tuple:

```
let (x, y, z) = tup;
println!("The value of y is: {}", y);
```

We can also access a tuple element by using period (.) followed by the index of the value:

```rust
let x: (i32, f64, u8) = (500, 6.4, 1);

let five_hundred = x.0;

let six_point_four = x.1;

let one = x.2;
```

#### 2. Array

Every element of an array must be of the same type. Arrays are useful when we know the number of elements are not going to change.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

```rust
let a = [1; 3]; // [1, 1, 1]
```

## Functions


