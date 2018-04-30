//! fricgan is an input/output library for byte based manipulations, without
//! requiring std.  It has additional features (where `std` is enabled), but
//! the core feature set is designed to provide the minimal types and support
//! for cross-language IO work.
//!
//! At this time it does not implement byte order manipulation, as the std
//! primitives for the integral types support `swap_bytes`, `{to,from}_be`,
//! and `{to,from}_le`.  (If you are implementing without std then presumably
//! you already know how to do this, and understand why it would just clutter
//! this library up).

#![warn(missing_docs)]
#![cfg_attr(not(feature="std"), no_std)] 

#[cfg(feature="std")]
extern crate core;

#[cfg(test)]
extern crate tempdir;

#[cfg(
    any(
        feature="io-i8",
        feature="io-u8",
        feature="io-i16",
        feature="io-u16",
        feature="io-f32",
        feature="io-i32",
        feature="io-u32",
        feature="io-f64",
        feature="io-i64",
        feature="io-u64",
    )
)]
use core::mem::size_of;

#[cfg(
    any(
        feature="io-i8",
        feature="io-u8",
        feature="io-i16",
        feature="io-u16",
        feature="io-f32",
        feature="io-i32",
        feature="io-u32",
        feature="io-f64",
        feature="io-i64",
        feature="io-u64",
    )
)]
use core::slice::from_raw_parts_mut;

#[cfg(feature="unsafe")]
use core::ptr;

#[cfg(feature="std")]
use std::io::Read as StandardRead;

#[cfg(feature="std")]
use std::io::Write as StandardWrite;


#[cfg(feature="num-traits")]
extern crate num_traits;

#[cfg(feature="num-traits")]
use num_traits::{cast::{FromPrimitive, ToPrimitive}, sign::Unsigned};

// ----------------------------------------------------------------------
// IO (and IO implementations)
// ----------------------------------------------------------------------

/// IO is a simple input/output that operates on bytes.  It isn't
/// designed to be fancy, it's just designed to be fast.  There is
/// likely room for improvement, but for now the implementations
/// work and provide fairly great speed.
///
/// The prefix `fio` is for fricgan-input-output, and is used to
/// prevent name collisions.
pub trait IO {
    /// Writes bytes to a byte buffer.
    /// `self` is mutable because certain types need to step the
    /// internal index/offset value.
    ///
    /// This shall write to offset zero (`0`).
    ///
    /// The return value shall always be the number of bytes written.
    fn fio_write(&mut self, sink: &mut [u8]) -> usize;

    /// Reads bytes to a byte buffer.
    ///
    /// This shall read from offset zero (`0`).
    ///
    /// The return value shall always be the number of bytes read.
    fn fio_read(&mut self, source: &[u8]) -> usize;
}

// [u8] implementation reduces the overall complexity of the below,
// and centralises the safety check.  This block is the safer, slower,
// version, that's designed to keep people happy.
#[cfg(not(feature="unsafe"))]
impl IO for [u8] {
    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        #[cfg(feature="safety-checks")]
        assert!(self.len() <= sink.len());

        for i in 0..self.len() {
            sink[i] = self[i];
        }

        self.len()
    }

    fn fio_read(&mut self, source: &[u8]) -> usize {
        #[cfg(feature="safety-checks")]
        assert!(self.len() <= source.len());

        for i in 0..self.len() {
            self[i] = source[i];
        }

        self.len()
    }
}


// [u8] implementation reduces the overall complexity of the below,
// and centralises the safety check.
//
// This is the unsafe/faster implementation.  It still won't fail unless
// you pass in things of the wrong size (at which point safety-checks
// picks up on it if that's enabled).
#[cfg(feature="unsafe")]
impl IO for [u8] {
    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        #[cfg(feature="safety-checks")]
        assert!(self.len() <= sink.len());

        unsafe {
            ptr::copy_nonoverlapping(self.as_ptr(), sink.as_mut_ptr(), self.len());
        }
        self.len()
    }

    fn fio_read(&mut self, source: &[u8]) -> usize {
        #[cfg(feature="safety-checks")]
        assert!(self.len() <= source.len());

        unsafe {
            ptr::copy_nonoverlapping(source.as_ptr(), self.as_mut_ptr(), self.len());
        }
        self.len()
    }
}

#[cfg(feature="io-u8")]
impl IO for u8 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        *self = source[0];
        1
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        sink[0] = *self;
        1
    }
}

#[cfg(feature="io-u8")]
#[test]
fn test_io_u8() {
    let mut data: [u8; 4] = [0,1,2,3];
    let mut test: u8 = 5u8;

    // read
    assert_eq!(test.fio_read(&data[..]), 1);
    assert_eq!(test, data[0]);
    assert_eq!(test.fio_read(&data[1..]), 1);
    assert_eq!(test, data[1]);

    // write
    test = 0xFFu8;
    assert_eq!(test.fio_write(&mut data[..]), 1);
    assert_eq!(test, 0xFF);
    assert_eq!(data[0], 0xFF);
    test = 0xF0u8;
    assert_eq!(test.fio_write(&mut data[1..]), 1);
    assert_eq!(test, 0xF0);
    assert_eq!(data[1], 0xF0);
}

#[cfg(feature="io-i8")]
impl IO for i8 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        *self = source[0] as i8;
        1
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        sink[0] = *self as u8;
        1
    }
}

#[cfg(feature="io-i8")]
#[test]
fn test_io_i8() {
    let mut data: [u8; 4] = [0,1,2,3];
    let mut test: i8 = 5i8;

    // read
    assert_eq!(test.fio_read(&data[..]), 1);
    assert_eq!(test, data[0] as i8);
    assert_eq!(test.fio_read(&data[1..]), 1);
    assert_eq!(test, data[1] as i8);

    // write
    test = 0x7Fi8;
    assert_eq!(test.fio_write(&mut data[..]), 1);
    assert_eq!(test, 0x7F);
    assert_eq!(data[0] as i8, 0x7F);
    test = 0x70i8;
    assert_eq!(test.fio_write(&mut data[1..]), 1);
    assert_eq!(test, 0x70);
    assert_eq!(data[1] as i8, 0x70);
}

#[cfg(feature="io-u16")]
impl IO for u16 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_read(source)
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_write(sink)
    }
}

#[cfg(feature="io-u16")]
#[test]
fn test_io_u16() {
    let mut data: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let mut test: u16 = 0x7654u16;

    // read
    assert_eq!(test.fio_read(&data[..]), 2);
    #[cfg(target_endian = "little")]
    assert_eq!(test, data[0] as u16 + ((data[1] as u16) << 8));
    #[cfg(target_endian = "big")]
    assert_eq!(test, data[1] as u16 + ((data[0] as u16) << 8));

    // write
    assert_eq!(test.fio_write(&mut data[2..]), 2);
    assert_eq!(data[0], data[2]);
    assert_eq!(data[1], data[3]);
}

#[cfg(feature="io-i16")]
impl IO for i16 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_read(source)
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_write(sink)
    }
}

#[cfg(feature="io-i16")]
#[test]
fn test_io_i16() {
    let mut data: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let mut test: i16 = 0x7654i16;

    // read
    assert_eq!(test.fio_read(&data[..]), 2);
    #[cfg(target_endian = "little")]
    assert_eq!(test, data[0] as i16 + ((data[1] as i16) << 8));
    #[cfg(target_endian = "big")]
    assert_eq!(test, data[1] as i16 + ((data[0] as i16) << 8));

    // write
    assert_eq!(test.fio_write(&mut data[2..]), 2);
    assert_eq!(data[0], data[2]);
    assert_eq!(data[1], data[3]);
}

#[cfg(feature="io-u32")]
impl IO for u32 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_read(source)
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_write(sink)
    }
}

#[cfg(feature="io-u32")]
#[test]
fn test_io_u32() {
    let mut data: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let mut test: u32 = 0x76543210u32;

    // read
    assert_eq!(test.fio_read(&data[..]), 4);
    #[cfg(target_endian = "little")]
    assert_eq!(test,
          data[0] as u32 +
        ((data[1] as u32) << 8) + 
        ((data[2] as u32) << 16) + 
        ((data[3] as u32) << 24));
    #[cfg(target_endian = "big")]
    assert_eq!(test, 
          data[3] as u32 +
        ((data[2] as u32) << 8) + 
        ((data[1] as u32) << 16) + 
        ((data[0] as u32) << 24));

    // write
    assert_eq!(test.fio_write(&mut data[4..]), 4);
    assert_eq!(data[0], data[4]);
    assert_eq!(data[1], data[5]);
    assert_eq!(data[2], data[6]);
    assert_eq!(data[3], data[7]);
}

#[cfg(feature="io-i32")]
impl IO for i32 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_read(source)
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_write(sink)
    }
}

#[cfg(feature="io-i32")]
#[test]
fn test_io_i32() {
    let mut data: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let mut test: i32 = 0x76543210i32;

    // read
    assert_eq!(test.fio_read(&data[..]), 4);
    #[cfg(target_endian = "little")]
    assert_eq!(test,
          data[0] as i32 +
        ((data[1] as i32) << 8) + 
        ((data[2] as i32) << 16) + 
        ((data[3] as i32) << 24));
    #[cfg(target_endian = "big")]
    assert_eq!(test, 
          data[3] as i32 +
        ((data[2] as i32) << 8) + 
        ((data[1] as i32) << 16) + 
        ((data[0] as i32) << 24));

    // write
    assert_eq!(test.fio_write(&mut data[4..]), 4);
    assert_eq!(data[0], data[4]);
    assert_eq!(data[1], data[5]);
    assert_eq!(data[2], data[6]);
    assert_eq!(data[3], data[7]);
}

#[cfg(feature="io-f32")]
impl IO for f32 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_read(source)
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_write(sink)
    }
}


#[cfg(feature="io-f32")]
#[test]
fn test_io_f32() {
    let mut data: [u8; 8] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    ];
    let mut test: f32 = 381.5476213f32;
    let mut test2 : f32 = 0.0f32;

    // Paranoia is fun
    assert_ne!(test, test2);

    // write
    assert_eq!(test.fio_write(&mut data[..]), 4);
    assert_ne!(test, test2);

    // read back
    assert_eq!(test2.fio_read(&data[..]), 4);
    assert_eq!(test, test2);
}


#[cfg(feature="io-u64")]
impl IO for u64 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_read(source)
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_write(sink)
    }
}

#[cfg(feature="io-u64")]
#[test]
fn test_io_u64() {
    let mut data: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
    ];
    let mut test: u64 = 0x7654321076543210u64;

    // read
    assert_eq!(test.fio_read(&data[..]), 8);
    #[cfg(target_endian = "little")]
    assert_eq!(test,
          data[0] as u64 +
        ((data[1] as u64) << 8) + 
        ((data[2] as u64) << 16) + 
        ((data[3] as u64) << 24) +
        ((data[4] as u64) << 32) +
        ((data[5] as u64) << 40) +
        ((data[6] as u64) << 48) +
        ((data[7] as u64) << 56)
    );
    #[cfg(target_endian = "big")]
    assert_eq!(test,
          data[7] as u64 +
        ((data[6] as u64) << 8) + 
        ((data[5] as u64) << 16) + 
        ((data[4] as u64) << 24) +
        ((data[3] as u64) << 32) +
        ((data[2] as u64) << 40) +
        ((data[1] as u64) << 48) +
        ((data[0] as u64) << 56)
    );

    // write
    assert_eq!(test.fio_write(&mut data[8..]), 8);
    assert_eq!(data[0], data[8]);
    assert_eq!(data[1], data[9]);
    assert_eq!(data[2], data[10]);
    assert_eq!(data[3], data[11]);
    assert_eq!(data[4], data[12]);
    assert_eq!(data[5], data[13]);
    assert_eq!(data[6], data[14]);
    assert_eq!(data[7], data[15]);
}

#[cfg(feature="io-i64")]
impl IO for i64 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_read(source)
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_write(sink)
    }
}

#[cfg(feature="io-i64")]
#[test]
fn test_io_i64() {
    let mut data: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
    ];
    let mut test: i64 = 0x7654321076543210i64;

    // read
    assert_eq!(test.fio_read(&data[..]), 8);
    #[cfg(target_endian = "little")]
    assert_eq!(test,
          data[0] as i64 +
        ((data[1] as i64) << 8) + 
        ((data[2] as i64) << 16) + 
        ((data[3] as i64) << 24) +
        ((data[4] as i64) << 32) +
        ((data[5] as i64) << 40) +
        ((data[6] as i64) << 48) +
        ((data[7] as i64) << 56)
    );
    #[cfg(target_endian = "big")]
    assert_eq!(test,
          data[7] as i64 +
        ((data[6] as i64) << 8) + 
        ((data[5] as i64) << 16) + 
        ((data[4] as i64) << 24) +
        ((data[3] as i64) << 32) +
        ((data[2] as i64) << 40) +
        ((data[1] as i64) << 48) +
        ((data[0] as i64) << 56)
    );

    // write
    assert_eq!(test.fio_write(&mut data[8..]), 8);
    assert_eq!(data[0], data[8]);
    assert_eq!(data[1], data[9]);
    assert_eq!(data[2], data[10]);
    assert_eq!(data[3], data[11]);
    assert_eq!(data[4], data[12]);
    assert_eq!(data[5], data[13]);
    assert_eq!(data[6], data[14]);
    assert_eq!(data[7], data[15]);
}

#[cfg(feature="io-f64")]
impl IO for f64 {
    fn fio_read(&mut self, source: &[u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_read(source)
    }

    fn fio_write(&mut self, sink: &mut [u8]) -> usize {
        let me: &mut [u8] = unsafe {
            from_raw_parts_mut(
                self as *mut Self as *mut u8,
                size_of::<Self>()
            )
        };
        me.fio_write(sink)
    }
}

#[cfg(feature="io-f64")]
#[test]
fn test_io_f64() {
    let mut data: [u8; 8] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    ];

    let mut test: f64 = 381.5476213f64;
    let mut test2 : f64 = 0.0f64;

    // Paranoia is fun
    assert_ne!(test, test2);

    // write
    assert_eq!(test.fio_write(&mut data[..]), 8);
    assert_ne!(test, test2);

    // read back
    assert_eq!(test2.fio_read(&data[..]), 8);
    assert_eq!(test, test2);
}

// ----------------------------------------------------------------------
// Standard integration
// ----------------------------------------------------------------------

/// Perform a std::io::Read operation on a fio typed value.
#[cfg(feature="std")]
pub fn fio_read<T: IO + Sized, R: StandardRead>(object: &mut T, source: &mut R) -> usize {
    let obj: &mut [u8] = unsafe {
        from_raw_parts_mut(
            object as *mut T as *mut u8,
            size_of::<T>()
        )
    };
    
    source.read_exact(&mut obj[..]).expect("failed to read");
    size_of::<T>()
}

/// Performs a std::io::Write operation on a fio typed value.
#[cfg(feature="std")]
pub fn fio_write<T: IO + Sized, W: StandardWrite>(object: &mut T, sink: &mut W) -> usize {
    let obj: &mut [u8] = unsafe {
        from_raw_parts_mut(
            object as *mut T as *mut u8,
            size_of::<T>()
        )
    };
    
    if sink.write_all(&mut obj[..]).is_ok() {
        size_of::<T>()
    } else {
        0
    }
}

#[cfg(all(feature="io-i32", feature="std"))]
#[test]
fn test_read_write_i32() {
    use tempdir::TempDir;    
    use std::fs::OpenOptions;
    use std::io::{Seek, SeekFrom};

    let tmp_dir = TempDir::new("fricgan").unwrap();
    let file_path = tmp_dir.path().join("i32");
    let mut tmp_file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(file_path).unwrap();

    let data: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let mut test: i32 = 0;

    // Pull four bytes
    assert_eq!(test.fio_read(&data[..]), 4);

    // Write
    assert_eq!(fio_write(&mut test, &mut tmp_file), 4);

    // Test
    assert_eq!(tmp_file.seek(SeekFrom::Current(0)).unwrap(), 4);

    // Pull the next four bytes
    assert_eq!(test.fio_read(&data[4..]), 4);

    // Write
    assert_eq!(fio_write(&mut test, &mut tmp_file), 4);

    // Test
    assert_eq!(tmp_file.seek(SeekFrom::Current(0)).unwrap(), 8);

    // Seek back
    assert_eq!(tmp_file.seek(SeekFrom::Start(0)).unwrap(), 0);

    // Test
    assert_eq!(tmp_file.seek(SeekFrom::Current(0)).unwrap(), 0);

    // Read back
    assert_eq!(fio_read(&mut test, &mut tmp_file), 4);

    #[cfg(target_endian = "little")]
    assert_eq!(test,
          data[0] as i32 +
        ((data[1] as i32) << 8) + 
        ((data[2] as i32) << 16) + 
        ((data[3] as i32) << 24));
    #[cfg(target_endian = "big")]
    assert_eq!(test, 
          data[3] as i32 +
        ((data[2] as i32) << 8) + 
        ((data[1] as i32) << 16) + 
        ((data[0] as i32) << 24));

    // Read back
    assert_eq!(fio_read(&mut test, &mut tmp_file), 4);

    #[cfg(target_endian = "little")]
    assert_eq!(test,
          data[4] as i32 +
        ((data[5] as i32) << 8) + 
        ((data[6] as i32) << 16) + 
        ((data[7] as i32) << 24));
    #[cfg(target_endian = "big")]
    assert_eq!(test, 
          data[6] as i32 +
        ((data[5] as i32) << 8) + 
        ((data[4] as i32) << 16) + 
        ((data[3] as i32) << 24));

    drop(tmp_file);
    tmp_dir.close().unwrap();
}

// ----------------------------------------------------------------------
// Variable Length Quantity (Numeric Only)
// ----------------------------------------------------------------------

/// VLQ is Variable Length Quantity, which, in this context, provides
/// vlq read/write values to the underlying value.
#[cfg(feature="vlq")]
pub trait VLQ {
    /// Writes bytes to a byte buffer.
    /// This shall write to offset zero (`0`).
    /// 
    /// The return value shall always be the number of bytes written.
    fn vlq_write(&self, sink: &mut [u8]) -> usize;

    /// Reads bytes to a byte buffer.
    /// 
    /// This shall read from offset zero (`0`).
    /// 
    /// The return value shall always be the number of bytes read.
    fn vlq_read(&mut self, source: &[u8]) -> usize;
}

#[cfg(feature="vlq-32")]
impl VLQ for u32 {
    fn vlq_read(&mut self, source: &[u8]) -> usize {
        *self = 0;
        let mut bits: usize = 0;
        let mut i: usize = 0;
        while bits != 35 {
            let b: u8 = source[i];
            *self += ((b & 127) as Self) << bits;
            bits += 7;
            if (b & 128) == 0 {
                break;
            }
            i += 1;
        }
        i
    }

    fn vlq_write(&self, sink: &mut [u8]) -> usize {
        let mut remainder = *self;
        let mut i: usize = 0;
        while remainder >= 128 {
            let b: u8 = remainder as u8 | 128;
            sink[i] = b;
            remainder = remainder >> 7;
            i += 1;
        }
        let b: u8 = remainder as u8;
        sink[i] = b;
        i
    }
}

#[cfg(feature="vlq-64")]
impl VLQ for u64 {
    fn vlq_read(&mut self, source: &[u8]) -> usize {
        *self = 0;
        let mut bits: usize = 0;
        let mut i: usize = 0;
        while bits != 71 {
            let b: u8 = source[i];
            *self += ((b & 127) as Self) << bits;
            bits += 7;
            if (b & 128) == 0 {
                break;
            }
            i += 1;
        }
        i
    }

    fn vlq_write(&self, sink: &mut [u8]) -> usize {
        let mut remainder = *self;
        let mut i: usize = 0;
        while remainder >= 128 {
            let b: u8 = remainder as u8 | 128;
            sink[i] = b;
            remainder = remainder >> 7;
            i += 1;
        }
        let b: u8 = remainder as u8;
        sink[i] = b;
        i
    }
}

// ----------------------------------------------------------------------
// Strings
// ----------------------------------------------------------------------

/// FricganString defines the string interface of `fio_string_read` and
/// `fio_string_write`.  By default it is implemented for `String`,
/// though it can theoretically work on any container class.
#[cfg(feature="io-string")]
pub trait FricganString {
    /// Read a String value using an unsigned integer.
    /// 
    /// This can be used for arbitrary types, but it is designed
    /// to operate on String.
    fn fio_string_read<V>(&mut self, source: &[u8]) -> usize
    where V: ToPrimitive + FromPrimitive + Unsigned + IO;

    /// Writes the underlying string to sink prefixed with an
    /// unsigned integer indicating length.
    /// 
    /// This can be used for arbitrary types, but it is designed
    /// to operate on String.
    fn fio_string_write<V>(&mut self, sink: &mut [u8]) -> usize
    where V: ToPrimitive + FromPrimitive + Unsigned + IO;
}

#[cfg(feature="io-string")]
impl FricganString for String {
    fn fio_string_read<V>(&mut self, source: &[u8]) -> usize
    where V: ToPrimitive + FromPrimitive + Unsigned + IO {
        let mut length: V = V::from_usize(0).unwrap();
        let mut read = length.fio_read(source);
        let length_usize : usize = V::to_usize(&length).unwrap();
        self.reserve_exact(length_usize);
        read += unsafe {
            let vv: &mut Vec<u8> = self.as_mut_vec();
            vv.set_len(length_usize);
            let v: &mut [u8] = vv.as_mut_slice();
            v.fio_read(&source[size_of::<V>()..])
        };
        read
    }

    fn fio_string_write<V>(&mut self, sink: &mut [u8]) -> usize
    where V: ToPrimitive + FromPrimitive + Unsigned + IO {
        let mut length: V = V::from_usize(self.len()).unwrap();
        let mut written = length.fio_write(sink);
        let length_usize : usize = V::to_usize(&length).unwrap();
        written += unsafe {
            let vv: &mut Vec<u8> = self.as_mut_vec();
            vv.set_len(length_usize);
            let v: &mut [u8] = vv.as_mut_slice();
            v.fio_write(&mut sink[size_of::<V>()..])
        };
        written
    }
}

#[cfg(all(feature="io-string",feature="io-u32"))]
#[test]
fn test_io_string() {

    let mut a = "This is a sample string to be written out and then restored"
        .to_owned();

    let mut b : String = "".to_owned();

    let mut _v = Vec::<u8>::with_capacity(a.len() + 4);
    unsafe {
        _v.set_len(a.len() + 4);
    }
    let v = _v.as_mut_slice();
    println!("a len: {}", a.len());
    println!("b len: {}", b.len());
    println!("v len: {}", v.len());
    
    let mut l = a.fio_string_write::<u32>(&mut v[..]);

    assert_eq!(l, a.len() + 4);

    for i in 0..a.len() {
        assert_eq!(a.as_bytes()[i], v[i+4]);
    }

    l = b.fio_string_read::<u32>(&v[..]);
    assert_eq!(l, a.len() + 4);

    for i in 0..b.len() {
        assert_eq!(b.as_bytes()[i], v[i+4]);
    }

    println!("a: {}\nb: {}", a.as_str(), b.as_str());
}

// ----------------------------------------------------------------------
// VLQ for Strings
// ----------------------------------------------------------------------

/// VLQString is the trait for defining VLQ-indexed strings in storage.
/// Fricgan provides a generic VLQString implementation for `String`,
/// allowing either `u32` or `u64` to be passed into it (depending on
/// what is enabled in the features, or what is implemented externally).
/// 
/// It can theoretically be used for any container class.
#[cfg(feature="vlq-string")]
pub trait VLQString {
    /// Read a String value using a VLQ typed unsigned integer.
    /// 
    /// This can be used for arbitrary types, but it is designed
    /// to operate on String.
    fn vlq_string_read<V: Unsigned>(&mut self, source: &[u8]) -> usize
    where V: ToPrimitive + FromPrimitive + Unsigned + VLQ;

    /// Writes the underlying value to the sink as a string prefixed
    /// with a VLQ typed unsigned integer preceding it.
    /// 
    /// This can be used for arbitrary types, but it is designed
    /// to operate on String.
    fn vlq_string_write<V>(&mut self, sink: &mut [u8]) -> usize
    where V: ToPrimitive + FromPrimitive + Unsigned + VLQ;
}

#[cfg(feature="vlq-string")]
impl VLQString for String {
    fn vlq_string_read<V>(&mut self, source: &[u8]) -> usize
    where V: ToPrimitive + FromPrimitive + Unsigned + VLQ {
        let mut length: V = V::from_usize(0).unwrap();
        let mut read = length.vlq_read(source);
        let length_usize : usize = read;
        self.reserve_exact(length_usize);
        read += unsafe {
            let vv: &mut Vec<u8> = self.as_mut_vec();
            vv.set_len(length_usize);
            let v: &mut [u8] = vv.as_mut_slice();
            v.fio_read(&source[length_usize..])
        };
        read
    }

    fn vlq_string_write<V>(&mut self, sink: &mut [u8]) -> usize
    where V: ToPrimitive + FromPrimitive + Unsigned + VLQ {
        let length: V = V::from_usize(self.len()).unwrap();
        let mut written = length.vlq_write(sink);
        let length_usize : usize = written;
        written += unsafe {
            let vv: &mut Vec<u8> = self.as_mut_vec();
            vv.set_len(length_usize);
            let v: &mut [u8] = vv.as_mut_slice();
            v.fio_write(&mut sink[length_usize..])
        };
        written
    }
}

#[cfg(all(feature="vlq-string",feature="vlq-32"))]
#[test]
fn test_vlq_string() {

    let mut a = "This is a sample string to be written out and then restored"
        .to_owned();

    let mut b : String = "".to_owned();

    let mut _v = Vec::<u8>::with_capacity(a.len() + 4);
    unsafe {
        _v.set_len(a.len() + 4);
    }
    let v = _v.as_mut_slice();
    println!("a len: {}", a.len());
    println!("b len: {}", b.len());
    println!("v len: {}", v.len());
    
    let mut l = a.vlq_string_write::<u32>(&mut v[..]);

    // Offset is unknown because it's VLQ (not so much in a test,
    // but the point remains here that the only test variable should
    // be the random string above)
    let mut u : u32 = 0;
    u.vlq_read(&v[..]);

    // o is for offset.
    let o = usize::from_u32(0).unwrap();

    assert_eq!(l, a.len() + o);

    for i in 0..a.len() {
        assert_eq!(a.as_bytes()[i], v[i+o]);
    }

    l = b.vlq_string_read::<u32>(&v[..]);
    assert_eq!(l, a.len() + o);

    for i in 0..b.len() {
        assert_eq!(b.as_bytes()[i], v[i+o]);
    }

    println!("a: {}\nb: {}", a.as_str(), b.as_str());
}