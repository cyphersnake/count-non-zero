# CountToNonZero

## Overview
The CountToNonZeroExt crate provides an extension trait for Rust's standard `Iterator` trait, enabling the counting of non-zero elements in iterators in a way that returns an `Option<NonZeroUsize>`. This approach optimally integrates with Rust's type system to offer a compile-time guarantee that the result, when present, is indeed non-zero. This functionality is particularly useful in scenarios where distinguishing between zero and non-zero counts is critical and can lead to more efficient code by leveraging the NonZeroUsize type's ability to optimize memory layouts and conditional logic.

## Features
Extends the `Iterator` trait with the `count_to_non_zero` method.
The `count_to_non_zero` method counts the elements for which the iterator yields values, returning an `Option<NonZeroUsize>`.

## Usage

```rust
use std::num::NonZeroUsize;

use count_to_non_zero::CountToNonZeroExt;

fn main() {
    let data = vec![1, 2, 0, 4];
    let count = data.into_iter().filter(|el| el % 2 == 0).count_to_non_zero();
    
    match count {
        Some(non_zero_count) => println!("Non-zero count of elements: {}", non_zero_count),
        None => println!("Iterator is empty"),
    }
}
```
