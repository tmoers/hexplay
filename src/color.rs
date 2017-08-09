//! Provides helpers for generating colors for use in HexViewBuilder printing,
//! as well as some reexports of the underlying color crate, `termcolor`

use std::io::{self, Write};
use std::ops::Range;

use termcolor::WriteColor;

pub use termcolor::Color as Color;
pub use termcolor::ColorSpec as Spec;

/// A vector of `(ColorSpec, Range)` values to print
pub type Colors = Vec<(Spec, Range<usize>)>;

pub(crate) struct ColorlessString(pub String);

impl Write for ColorlessString {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use std::str;
        let string = str::from_utf8(buf).unwrap();
        self.0 += string;
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl WriteColor for ColorlessString {
    fn supports_color(&self) -> bool {
        false
    }
    fn set_color(&mut self, _spec: &Spec) -> io::Result<()> {
        Ok(())
    }
    fn reset (&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(crate) struct ColorRange<'a> {
    colors: &'a Colors,
    offset: usize,
    idx: usize,
}

impl<'a> Clone for ColorRange<'a> {
    fn clone(&self) -> Self {
        ColorRange {
            colors: self.colors,
            offset: self.offset,
            idx: self.idx,
        }
    }
}

impl<'a> ColorRange<'a> {
    pub fn new(colors: &'a Colors) -> Self {
        ColorRange {
            colors: colors,
            offset: 0,
            idx: 0,
        }
    }
    pub fn update_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    pub fn get(&mut self, idx: usize) -> Option<Spec> {
        while self.idx < self.colors.len() {
            let (rgb, range) = self.colors[self.idx].clone();
            let offset = self.offset + idx;
            if offset >= range.start && offset < range.end {
                return Some(rgb)
            } else if offset <= range.start { // for non-contiguous colors
                return None
            } else {
                self.idx += 1;
            }
        }
        None
    }
}

macro_rules! make_color {
    ($name:ident, $name_bold:ident, $color:ident) => {
        /// Creates the appropriate ColorSpec
        pub fn $name() -> Spec {
            Spec::new().set_bold(false).set_fg(Some(Color::$color)).clone()
        }
        /// Creates the appropriate ColorSpec, in bold
        pub fn $name_bold () -> Spec {
            Spec::new().set_bold(true).set_fg(Some(Color::$color)).clone()
        }
    }
}

make_color!(red, red_bold, Red);
make_color!(blue, blue_bold, Blue);
make_color!(green, green_bold, Green);
make_color!(yellow, yellow_bold, Yellow);
make_color!(magenta, magenta_bold, Magenta);
make_color!(black, black_bold, Black);
make_color!(cyan, cyan_bold, Cyan);
make_color!(white, white_bold, White);
