# Unsigned 2-10-10-10 vector

[![Version](https://img.shields.io/crates/v/vec-2-10-10-10.svg)](https://crates.io/crates/vec-2-10-10-10)
[![Build Status](https://travis-ci.org/Nercury/vec-2-10-10-10-rs.svg?branch=master)](https://travis-ci.org/Nercury/vec-2-10-10-10-rs)

This is 32-bit 4-dimensional vector, where the first dimension has 2 bits, and
the last 3 dimensions have 10 bits each. It is useful for representing color with 
an alpha, where the alpha does not require much precision.

It is compatible with `GL_UNSIGNED_INT_2_10_10_10_REV` in OpenGL.

## Example

```rust
extern crate vec_2_10_10_10;

fn main() {
    let value = vec_2_10_10_10::Vector::new(0.444, 0.555, 0.666, 0.2);
    
    assert!(approx_equal(value.x(), 0.444));
    assert!(approx_equal(value.y(), 0.555));
    assert!(approx_equal(value.z(), 0.666));
    
    // 2 bits means only possible values are 0, 0.3(3), 0.6(6) and 1.
    assert!(approx_equal(value.w(), 0.333));
}

fn approx_equal(a: f32, b: f32) -> bool {
    const DELTA: f32 = 0.001;
    a > b - DELTA && a < b + DELTA
}
```

## Installation

In your `Cargo.toml`, add:

```toml
vec-2-10-10-10 = "0.1"
```

Add reference to the crate:

```rust
extern crate vec_2_10_10_10;
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.