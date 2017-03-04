use itertools::Itertools;
use std::fmt::{Formatter, Result};
use std;

use byte_mapping;

/// The HexView struct represents the configuration of how to display the data.
pub struct HexView<'a> {
    address_offset: usize,
    codepage: &'a [char],
    data: &'a [u8],
    row_width: usize,
}

impl<'a> HexView<'a> {
    pub fn new(data: &[u8]) -> HexView {
        HexView {
            address_offset: 0,
            codepage: &byte_mapping::CODEPAGE_0850,
            data: data,
            row_width: 16,
        }
    }
}

/// A builder for the [HexView](struct.HexView.html) struct.
pub struct HexViewBuilder<'a> {
    hex_view: HexView<'a>,
}

impl<'a> HexViewBuilder<'a> {
    pub fn new(data: &[u8]) -> HexViewBuilder {
        HexViewBuilder {
            hex_view: HexView::new(&data)
        }
    }

    pub fn address_offset(mut self, offset: usize) -> HexViewBuilder<'a> {
        self.hex_view.address_offset = offset;
        self
    }

    pub fn codepage<'b: 'a>(mut self, codepage: &'b [char]) -> HexViewBuilder<'a> {
        self.hex_view.codepage = codepage;
        self
    }

    pub fn row_width(mut self, width: usize) -> HexViewBuilder<'a> {
        self.hex_view.row_width = width;
        self
    }

    pub fn finish(self) -> HexView<'a> {
        self.hex_view
    }
}

fn fmt_byte_as_hex(f: &mut Formatter, optional_byte: &Option<u8>) -> Result {
    match *optional_byte {
        Some(ref byte) => write!(f, "{:02X}", byte),
        None => write!(f, "  "),
    }
}

fn fmt_byte_as_char(f: &mut Formatter, cp: &[char], optional_byte: &Option<u8>) -> Result {
    match *optional_byte {
        Some(ref byte) => write!(f, "{}", byte_mapping::as_char(*byte, cp)),
        None => write!(f, "  "),
    }
}

/*
fn fmt_optional_bytes_line<I>(f: &mut Formatter, address: usize, cp: &[char], bytes: &I) -> Result where I: itertools::Itertools {
    write!(f, "{:0width$X}", address, width = 8)?;

    write!(f, "  ")?;
    for &byte in &bytes.into_iter() {
        fmt_byte_as_hex(f, &byte)?;
    }
    write!(f, "  ")?;

    write!(f, "| ")?;
    //for &byte in bytes.iter() {
        //fmt_byte_as_char(f, cp, &byte)?;
    //}
    write!(f, " |")?;

    Ok(())
}
*/
fn calculate_begin_padding(address_offset: usize, row_width: usize) -> usize {
    debug_assert!(row_width != 0, "A zero row width is can not be used to calculate the begin padding");
    address_offset % row_width
}

fn calculate_end_padding(data_size: usize, row_width: usize) -> usize {
    debug_assert!(row_width != 0, "A zero row width is can not be used to calculate the end padding");
    (row_width - data_size % row_width) % row_width
}


impl<'a> std::fmt::Display for HexView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        if self.row_width == 0 {
            write!(f, "Invalid HexView::width")?;
            return Err(std::fmt::Error);
        }

        let begin_padding = calculate_begin_padding(self.address_offset, self.row_width);
        let end_padding = calculate_end_padding(begin_padding + self.data.len(), self.row_width);
        let mut address = self.address_offset - begin_padding;

        let begin_padding_iter = std::iter::repeat(None).take(begin_padding);
        let end_padding_iter = std::iter::repeat(None).take(end_padding);
        let data_iter = self.data.iter().map(|value| { Some(value) });

        let total_iter = std::iter::empty()
            .chain(begin_padding_iter)
            .chain(data_iter)
            .chain(end_padding_iter);

        for &chunk in &total_iter.chunks(self.row_width).into_iter() {
            //fmt_optional_bytes_line(f, address, self.codepage, &chunk);
            for byte in &chunk.into_iter() {

            }
            address += self.row_width;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_begin_padding() {
        // Rust 1.13 needs the fully qualified name here
        assert_eq!(super::calculate_begin_padding(0, 16), 0);
        assert_eq!(super::calculate_begin_padding(16, 16), 0);
        assert_eq!(super::calculate_begin_padding(54, 16), 6);
    }

    #[test]
    fn a_full_line_is_formatted_as_expected() {
        let data: Vec<u8> = (0x40..0x40 + 0xF + 1).collect();

        let row_view = HexViewBuilder::new(&data)
            .row_width(data.len())
            .finish();

        let result = format!("{}", row_view);

        assert_eq!(result, "00000000  40 41 42 43 44 45 46 47 48 49 4A 4B 4C 4D 4E 4F  | @ABCDEFGHIJKLMNO |");
    }

    #[test]
    fn an_incomplete_line_is_padded_on_the_right() {
        let data = ['a' as u8; 10];

        let row_view = HexViewBuilder::new(&data)
            .row_width(16)
            .finish();

        let result = format!("{}", row_view);

        assert_eq!(result, "00000000  61 61 61 61 61 61 61 61 61 61                    | aaaaaaaaaa       |");
    }

    #[test]
    fn an_unaligned_address_causes_padded_on_the_left() {
        let data = ['a' as u8; 11];

        let row_view = HexViewBuilder::new(&data)
            .address_offset(5)
            .row_width(16)
            .finish();

        let result = format!("{}", row_view);
        println!("{}", result);

        assert_eq!(result, "00000000                 61 61 61 61 61 61 61 61 61 61 61  |      aaaaaaaaaaa |");
    }

    #[test]
    fn an_unaligned_incomplete_line_causes_padding_on_both_sides() {
        let data = ['a' as u8; 8];

        let row_view = HexViewBuilder::new(&data)
            .address_offset(5)
            .row_width(16)
            .finish();

        let result = format!("{}", row_view);
        println!("{}", result);

        assert_eq!(result, "00000000                 61 61 61 61 61 61 61 61           |      aaaaaaaa    |");
    }

    #[test]
    fn decreasing_the_row_width_increases_the_total_character_count() {
        let data: Vec<u8> = (0..64).collect();

        let short_row_view = HexViewBuilder::new(&data).row_width(0).finish();
        let long_row_view = HexViewBuilder::new(&data).row_width(16).finish();

        let short_row_result = format!("{}", short_row_view);
        let long_row_result = format!("{}", long_row_view);

        assert!(short_row_result.len() < long_row_result.len());
    }

    #[test]
    fn the_address_offset_is_zero_by_default() {
        let data = [99; 16];

        let row_view = HexViewBuilder::new(&data)
            .row_width(16)
            .finish();

        let result = format!("{}", row_view);
        let address_offset_str = format!("{:X}", 0);

        assert!(result.contains(&address_offset_str));
    }

    #[test]
    fn the_address_offset_is_used_when_given() {
        let data = [0; 16];

        let address_offset = data.len() * 10;
        let row_view = HexViewBuilder::new(&data)
            .row_width(16)
            .address_offset(address_offset)
            .finish();

        let result = format!("{}", row_view);
        let address_offset_str = format!("{:X}", address_offset);

        assert!(result.contains(&address_offset_str));
    }

    #[test]
    fn the_address_offset_increases_by_the_row_width_for_each_row() {
        let data = [0; 16 * 5];

        let address_offset = data.len() * 10;
        let row_view = HexViewBuilder::new(&data)
            .row_width(16)
            .address_offset(address_offset)
            .finish();

        let result = format!("{}", row_view);
        let row_2_address_offset_str = format!("{:X}", address_offset + 2 * row_view.row_width);
        let row_4_address_offset_str = format!("{:X}", address_offset + 4 * row_view.row_width);

        assert!(result.contains(&row_2_address_offset_str));
        assert!(result.contains(&row_4_address_offset_str));
    }

    #[test]
    fn the_row_width_is_16_by_default() {
        let data = [0; 17];

        let one_line_result = format!("{}", HexViewBuilder::new(&data[0..16]).finish());
        let two_line_result = format!("{}", HexViewBuilder::new(&data[0..17]).finish());

        println!("{}", one_line_result);
        println!("{}", two_line_result);

        assert_eq!(1, one_line_result.lines().count());
        assert_eq!(2, two_line_result.lines().count());
    }

    #[test]
    fn all_characters_can_be_printed() {
        let data: Vec<u8> = (0u16..256u16).map(|v| v as u8).collect();

        let dump_view = HexViewBuilder::new(&data)
            .address_offset(20)
            .row_width(8)
            .finish();

        let result = format!("{}", dump_view);
        println!("{}", result);

        assert!(!result.is_empty());
    }
}
