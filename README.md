# Unsigned 2-10-10-10 vector

This is 32-bit 4-dimensional vector, where the first dimension has 2 bits, and
the last 3 dimensions have 10 bits each. It is useful for representing color with 
an alpha, where the alpha does not require much precision.

It is compatible with `GL_UNSIGNED_INT_2_10_10_10_REV` in OpenGL.

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