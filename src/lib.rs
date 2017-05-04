//! This crate provides a way to [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
//! a byte slice as as it is commonly done in a hex-editor.
//!
//! The configuration of the visualization are stored in the [HexView](struct.HexView.html),
//! struct, which can be easily constructed using the [HexViewBuilder](struct.HexViewBuilder.html).
//!
//! # Examples
//!
//! Usage is very simple, just build a `HexView` and use it for formatting:
//!
//! ```rust
//! use hexplay::HexViewBuilder;
//!
//! // The buffer we want to display
//! let data : Vec<u8> = (0u8..200u8).collect();
//!
//! // Build a new HexView using the provider builder
//! let view = HexViewBuilder::new(&data[40..72])
//!     .address_offset(40)
//!     .row_width(16)
//!     .finish();
//!
//! println!("{}", view);
//!
//! # let result = format!("{}", view);
//! # let mut lines = result.lines();
//! # assert_eq!("00000020                          28 29 2A 2B 2C 2D 2E 2F  |         ()*+,-./ |", lines.next().unwrap());
//! # assert_eq!("00000030  30 31 32 33 34 35 36 37 38 39 3A 3B 3C 3D 3E 3F  | 0123456789:;<=>? |", lines.next().unwrap());
//! # assert_eq!("00000040  40 41 42 43 44 45 46 47                          | @ABCDEFG         |", lines.next().unwrap());
//! ```
//!
//! This will result in the following output:
//!
//! ```text
//! 00000020                          28 29 2A 2B 2C 2D 2E 2F  |         ()*+,-./ |
//! 00000030  30 31 32 33 34 35 36 37 38 39 3A 3B 3C 3D 3E 3F  | 0123456789:;<=>? |
//! 00000040  40 41 42 43 44 45 46 47                          | @ABCDEFG         |
//! ```

mod byte_mapping;
mod format;

pub use byte_mapping::CODEPAGE_0850;
pub use byte_mapping::CODEPAGE_1252;
pub use byte_mapping::CODEPAGE_ASCII;
pub use format::HexView;
pub use format::HexViewBuilder;
