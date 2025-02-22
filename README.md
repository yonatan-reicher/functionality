# Functionality

[![Test](https://github.com/yonatan-reicher/functionality/workflows/Test/badge.svg)](https://github.com/yonatan-reicher/functionality/actions?query=workflow%3ATest)
[![Crates.io Version](https://img.shields.io/crates/v/functionality?logo=rust)](https://crates.io/crates/functionality)

Adds support for chaining functions in a functional way.

## Pipe

The following 9 methods are added to all types:

|       pipe syntax      |  traditional syntax equivalent |
|:----------------------:|:------------------------------:|
| `x.pipe(f)`            | `f(x)`                         |
| `x.pipe_ref(f)`        | `f(&x)`                        |
| `x.pipe_mut(f)`        | `f(&mut x)`                    |
| `x.pipe_as_ref(f)`     | `f(x.as_ref())`                |
| `x.pipe_as_mut(f)`     | `f(x.as_mut())`                |
| `x.pipe_deref(f)`      | `f(&x)`                        |
| `x.pipe_deref_mut(f)`  | `f(&mut x)`                    |
| `x.pipe_borrow(f)`     | `f(x.borrow())`                |
| `x.pipe_borrow_mut(f)` | `f(x.borrow_mut())`            |

These are imported directly from [the pipe-trait crate](https://github.com/KSXGitHub/pipe-trait).

### Example

```rust
use functionality::prelude::*;

let inc = |x| x + 1;
let double = |x| x + x;
let square = |x| x * x;
let a = (123i32).pipe(inc).pipe(double).pipe(square);
let b = square(double(inc(123i32)));
assert_eq!(a, b);
```

## Mutate

The method `.mutate(..)` is also added to all types.

### Example

```rust
use functionality::prelude::*;

let sorted = vec![3, 2, 1].mutate(|v| v.sort());
assert_eq!(sorted, vec![1, 2, 3]);
```
