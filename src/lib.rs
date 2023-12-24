// Copyright 2017 Nerijus Arlauskas
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! This is 32-bit 4-dimensional vector, where the first dimension has 2 bits, and
//! the last 3 dimensions have 10 bits each. It is useful for representing color with
//! an alpha, where the alpha does not require much precision.
//!
//! It is compatible with `GL_UNSIGNED_INT_2_10_10_10_REV` in OpenGL.
//!
//! ## Example
//!
//! ```rust
//! extern crate vec_2_10_10_10;
//!
//! fn main() {
//!     let value = vec_2_10_10_10::Vector::new(0.444, 0.555, 0.666, 0.2);
//!
//!     assert!(approx_equal(value.x(), 0.444));
//!     assert!(approx_equal(value.y(), 0.555));
//!     assert!(approx_equal(value.z(), 0.666));
//!
//!     // 2 bits means only possible values are 0, 0.3(3), 0.6(6) and 1.
//!     assert!(approx_equal(value.w(), 0.333));
//! }
//!
//! fn approx_equal(a: f32, b: f32) -> bool {
//!     const DELTA: f32 = 0.001;
//!     a > b - DELTA && a < b + DELTA
//! }
//! ```

use std::fmt;

/// Four dimensional 2-10-10-10 vector.
///
/// The binary data is mapped into floating point values from `0.0` to `1.0`.
/// The values outside this range are clamped.
///
/// The `w` dimension takes 2 bits, and can have values `0.0`, `0.3(3)`, `0.6(6)` and `1.0`.
/// The `x`, `y` and `z` dimensions take 10 bits, each.
///
/// The internal format is equivalent to `GL_UNSIGNED_INT_2_10_10_10_REV` OpenGL
/// vertex attribute type.
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Vector {
    data: u32,
}

impl Vector {
    /// Creates a new Vector.
    ///
    /// First `x`, `y`, `z` values are stored in 10-bits, each.
    /// The `w` value is stored in 2 bits.
    ///
    /// Everything is packed internally into 4 bytes.
    ///
    /// The stored values are a bit wonky _precisely_ because of low stored precision.
    ///
    /// ```
    /// let value = vec_2_10_10_10::Vector::new(0.444, 0.555, 0.666, 0.2);
    ///
    /// assert!(approx_equal(value.x(), 0.444));
    /// assert!(approx_equal(value.y(), 0.555));
    /// assert!(approx_equal(value.z(), 0.666));
    ///
    /// // 2 bits means only possible values are 0, 0.3(3), 0.6(6) and 1.
    /// assert!(approx_equal(value.w(), 0.333));
    ///
    /// fn approx_equal(a: f32, b: f32) -> bool {
    ///     const DELTA: f32 = 0.001;
    ///     a > b - DELTA && a < b + DELTA
    /// }
    /// ```
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector {
        let x = (clamp(x) * 1023f32).round() as u32;
        let y = (clamp(y) * 1023f32).round() as u32;
        let z = (clamp(z) * 1023f32).round() as u32;
        let w = (clamp(w) * 3f32).round() as u32;

        let mut c: u32 = 0;
        c |= w << 30;
        c |= z << 20;
        c |= y << 10;
        c |= x;

        Vector { data: c }
    }

    /// Creates a vector from raw 4-byte data.
    ///
    /// The vector can be used to inspect such data if it was created by other means.
    ///
    /// ```
    /// let other_value = *vec_2_10_10_10::Vector::new(0.444, 0.555, 0.666, 0.333).raw_value();
    /// let value = vec_2_10_10_10::Vector::from_raw(other_value);
    ///
    /// assert!(approx_equal(value.x(), 0.444));
    /// assert!(approx_equal(value.y(), 0.555));
    /// assert!(approx_equal(value.z(), 0.666));
    /// assert!(approx_equal(value.w(), 0.333));
    ///
    /// fn approx_equal(a: f32, b: f32) -> bool {
    ///     const DELTA: f32 = 0.001;
    ///     a > b - DELTA && a < b + DELTA
    /// }
    /// ```
    pub fn from_raw(data: u32) -> Vector {
        Vector { data }
    }

    /// Get `x` value.
    pub fn x(&self) -> f32 {
        (1023 & self.data) as f32 / 1023f32
    }

    /// Get `y` value.
    pub fn y(&self) -> f32 {
        ((1023 << 10 & self.data) >> 10) as f32 / 1023f32
    }

    /// Get `z` value.
    pub fn z(&self) -> f32 {
        ((1023 << 20 & self.data) >> 20) as f32 / 1023f32
    }

    /// Get `w` value.
    pub fn w(&self) -> f32 {
        ((0b11 << 30 & self.data) >> 30) as f32 / 3f32
    }

    /// Update `x` value.
    ///
    /// This changes internal 4-byte representation.
    ///
    /// ```
    /// let mut value = vec_2_10_10_10::Vector::new(0.0, 0.0, 0.0, 0.0);
    /// value.set_x(0.333);
    ///
    /// assert!(approx_equal(value.x(), 0.333));
    /// assert!(approx_equal(value.y(), 0.0));
    /// assert!(approx_equal(value.z(), 0.0));
    /// assert!(approx_equal(value.w(), 0.0));
    /// #
    /// # fn approx_equal(a: f32, b: f32) -> bool {
    /// #     const DELTA: f32 = 0.001;
    /// #     a > b - DELTA && a < b + DELTA
    /// # }
    /// ```
    pub fn set_x(&mut self, x: f32) {
        let x = (clamp(x) * 1023f32).round() as u32;
        let mut c: u32 = (3 << 30 | 1023 << 20 | 1023 << 10) & self.data;
        c |= x;
        self.data = c;
    }

    /// Update `y` value.
    ///
    /// This changes internal 4-byte representation.
    ///
    /// ```
    /// let mut value = vec_2_10_10_10::Vector::new(0.0, 0.0, 0.0, 0.0);
    /// value.set_y(0.333);
    ///
    /// assert!(approx_equal(value.x(), 0.0));
    /// assert!(approx_equal(value.y(), 0.333));
    /// assert!(approx_equal(value.z(), 0.0));
    /// assert!(approx_equal(value.w(), 0.0));
    /// #
    /// # fn approx_equal(a: f32, b: f32) -> bool {
    /// #     const DELTA: f32 = 0.001;
    /// #     a > b - DELTA && a < b + DELTA
    /// # }
    /// ```
    pub fn set_y(&mut self, y: f32) {
        let y = (clamp(y) * 1023f32).round() as u32;
        let mut c: u32 = (3 << 30 | 1023 << 20 | 1023) & self.data;
        c |= y << 10;
        self.data = c;
    }

    /// Update `z` value.
    ///
    /// This changes internal 4-byte representation.
    ///
    /// ```
    /// let mut value = vec_2_10_10_10::Vector::new(0.0, 0.0, 0.0, 0.0);
    /// value.set_z(0.333);
    ///
    /// assert!(approx_equal(value.x(), 0.0));
    /// assert!(approx_equal(value.y(), 0.0));
    /// assert!(approx_equal(value.z(), 0.333));
    /// assert!(approx_equal(value.w(), 0.0));
    /// #
    /// # fn approx_equal(a: f32, b: f32) -> bool {
    /// #     const DELTA: f32 = 0.001;
    /// #     a > b - DELTA && a < b + DELTA
    /// # }
    /// ```
    pub fn set_z(&mut self, z: f32) {
        let z = (clamp(z) * 1023f32).round() as u32;
        let mut c: u32 = (3 << 30 | 1023 << 10 | 1023) & self.data;
        c |= z << 20;
        self.data = c;
    }

    /// Update `x`, `y` and `z`.
    ///
    /// This changes internal 4-byte representation.
    ///
    /// ```
    /// let mut value = vec_2_10_10_10::Vector::new(0.0, 0.0, 0.0, 0.0);
    /// value.set_xyz(0.333, 0.444, 0.555);
    ///
    /// assert!(approx_equal(value.x(), 0.333));
    /// assert!(approx_equal(value.y(), 0.444));
    /// assert!(approx_equal(value.z(), 0.555));
    /// assert!(approx_equal(value.w(), 0.0));
    /// #
    /// # fn approx_equal(a: f32, b: f32) -> bool {
    /// #     const DELTA: f32 = 0.001;
    /// #     a > b - DELTA && a < b + DELTA
    /// # }
    /// ```
    pub fn set_xyz(&mut self, x: f32, y: f32, z: f32) {
        let x = (clamp(x) * 1023f32).round() as u32;
        let y = (clamp(y) * 1023f32).round() as u32;
        let z = (clamp(z) * 1023f32).round() as u32;
        let mut c: u32 = (3 << 30) & self.data;
        c |= z << 20;
        c |= y << 10;
        c |= x;
        self.data = c;
    }

    /// Update `w`.
    ///
    /// This changes internal 4-byte representation.
    ///
    /// ```
    /// let mut value = vec_2_10_10_10::Vector::new(0.0, 0.0, 0.0, 0.0);
    /// value.set_w(0.333);
    ///
    /// assert!(approx_equal(value.x(), 0.0));
    /// assert!(approx_equal(value.y(), 0.0));
    /// assert!(approx_equal(value.z(), 0.0));
    /// assert!(approx_equal(value.w(), 0.333));
    /// #
    /// # fn approx_equal(a: f32, b: f32) -> bool {
    /// #     const DELTA: f32 = 0.001;
    /// #     a > b - DELTA && a < b + DELTA
    /// # }
    /// ```
    pub fn set_w(&mut self, w: f32) {
        let w = (clamp(w) * 3f32).round() as u32;
        let mut c: u32 = (1023 << 20 | 1023 << 10 | 1023) & self.data;
        c |= w << 30;
        self.data = c;
    }

    /// Return raw internal value.
    pub fn raw_value(&self) -> u32 {
        self.data
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set()
            .entry(&self.x())
            .entry(&self.y())
            .entry(&self.z())
            .entry(&self.w())
            .finish()
    }
}

#[inline]
fn clamp(c: f32) -> f32 {
    if c < 0.0 {
        return 0.0;
    }
    if c > 1.0 {
        return 1.0;
    }
    c
}

