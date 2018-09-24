//
// Copyright 2018 Red Hat, Inc.
//
// Author: Nathaniel McCallum <npmccallum@redhat.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! The `voncount` crate provides utilities for keeping a running count of things.
//!
//! Like the lovable Count von Count from Sesame Street, the `voncount` crate loves to count things.
//! We provide the `Counter` trait which can be implemented on types which try to count things.
//! We also provide two structs which implement the `Counter` trait:
//!   * `ReadCounter`
//!   * `WriteCounter`

use std::io;

/// Describes types which count things. What they count is up to them.
pub trait Counter {
    /// Returns the current count of items counted.
    fn count(&self) -> usize;
}

/// Wraps any implementation of `std::io::Read` and counts the bytes read.
///
/// A `ReadCounter` instance wraps any implementation of `std::io::Read`. Since `ReadCounter` also
/// implements `std::io::Read` you can use it in place of the other implementation. The
/// `ReadCounter` will count the number of bytes read.
pub struct ReadCounter<'a, T: 'a + io::Read> {
    reader: &'a mut T,
    count: usize,
}

impl<'a, T: 'a + io::Read> From<&'a mut T> for ReadCounter<'a, T> {
    /// Creates a `ReadCounter` by wrapping any implementation of `std::io::Read`.
    ///
    /// The lifetime of this instance cannot be greater than the lifetime of the wrapped instance.
    fn from(value: &'a mut T) -> ReadCounter<'a, T> {
        ReadCounter {
            reader: value,
            count: 0,
        }
    }
}

impl<'a, T: 'a + io::Read> Counter for ReadCounter<'a, T> {
    /// Returns the number of bytes read so far.
    fn count(&self) -> usize {
        self.count
    }
}

impl<'a, T: 'a + io::Read> io::Read for ReadCounter<'a, T> {
    /// Proxies to the inner `read` function, counting the bytes read along the way.
    ///
    /// # Panics
    ///
    ///   1. When the underlying function panics.
    ///   2. If more than `usize::max_value()` bytes are read across all calls to `read`.
    ///
    /// # Errors
    ///
    /// This function will error only if the underlying function errors.
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, io::Error> {
        let size = self.reader.read(buffer)?;
        self.count += size;
        Ok(size)
    }
}

/// Wraps any implementation of `std::io::Write` and counts the bytes written.
///
/// A `WriteCounter` instance wraps any implementation of `std::io::Read`. Since `WriteCounter` also
/// implements `std::io::Write` you can use it in place of the other implementation. The
/// `WriteCounter` will count the number of bytes written.
pub struct WriteCounter<'a, T: 'a + io::Write> {
    writer: &'a mut T,
    count: usize,
}

impl<'a, T: 'a + io::Write> From<&'a mut T> for WriteCounter<'a, T> {
    /// Creates a `WriteCounter` by wrapping any implementation of `std::io::Write`.
    ///
    /// The lifetime of this instance cannot be greater than the lifetime of the wrapped instance.
    fn from(value: &'a mut T) -> WriteCounter<'a, T> {
        WriteCounter {
            writer: value,
            count: 0,
        }
    }
}

impl<'a, T: 'a + io::Write> Counter for WriteCounter<'a, T> {
    /// Returns the number of bytes written so far.
    fn count(&self) -> usize {
        self.count
    }
}

impl<'a, T: 'a + io::Write> io::Write for WriteCounter<'a, T> {
    /// Proxies to the inner `write` function, counting the bytes written along the way.
    ///
    /// # Panics
    ///
    ///   1. When the underlying function panics.
    ///   2. If more than `usize::max_value()` bytes are written across all calls to `write`.
    ///
    /// # Errors
    ///
    /// This function will error only if the underlying function errors.
    fn write(&mut self, buffer: &[u8]) -> Result<usize, io::Error> {
        let size = self.writer.write(buffer)?;
        self.count += size;
        Ok(size)
    }

    /// Proxies to the inner `flush` function.
    fn flush(&mut self) -> Result<(), io::Error> {
        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    const DATA: &'static [u8] = &[1u8, 2u8, 3u8];

    #[test]
    fn read() {
        let mut d = DATA;
        let mut r = ReadCounter::from(&mut d);

        for (i, v) in DATA.iter().enumerate() {
            let mut b = [0u8];

            assert_eq!(r.read(&mut b).unwrap(), 1);
            assert_eq!(r.count(), i + 1);
            assert_eq!(b[0], *v);
        }
    }

    #[test]
    fn write() {
        let mut b: Vec<u8> = Vec::new();

        {
            let mut w = WriteCounter::from(&mut b);

            for (i, v) in DATA.iter().enumerate() {
                assert_eq!(w.write(&[*v]).unwrap(), 1);
                assert_eq!(w.count(), i + 1);
            }
        }

        assert_eq!(&b[..], DATA);
    }
}
