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
        HexViewBuilder { hex_view: HexView::new(&data) }
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

#[derive(Default)]
struct Padding {
    left: usize,
    right: usize,
}

impl Padding {
    fn new(left_padding: usize, right_padding: usize) -> Padding {
        Padding {
            left: left_padding,
            right: right_padding,
        }
    }

    fn from_left(left_padding: usize) -> Padding {
        Padding {
            left: left_padding,
            right: 0,
        }
    }

    fn from_right(right_padding: usize) -> Padding {
        Padding {
            left: 0,
            right: right_padding,
        }
    }
}

fn fmt_bytes_as_hex(f: &mut Formatter, bytes: &[u8], padding: &Padding) -> Result {
    let mut separator = "";

    for _ in 0..padding.left {
        write!(f, "{}  ", separator)?;
        separator = " ";
    }

    for byte in bytes.iter() {
        write!(f, "{}{:02X}", separator, byte)?;
        separator = " ";
    }

    for _ in 0..padding.right {
        write!(f, "{}  ", separator)?;
        separator = " ";
    }

    Ok(())
}

fn fmt_bytes_as_char(f: &mut Formatter, cp: &[char], bytes: &[u8], padding: &Padding) -> Result {
    for _ in 0..padding.left {
        write!(f, " ")?;
    }

    for &byte in bytes.iter() {
        write!(f, "{}", byte_mapping::as_char(byte, cp))?;
    }

    for _ in 0..padding.right {
        write!(f, " ")?;
    }

    Ok(())
}

fn fmt_line(f: &mut Formatter,
            address: usize,
            cp: &[char],
            bytes: &[u8],
            padding: &Padding)
            -> Result {
    write!(f, "{:0width$X}", address, width = 8)?;

    write!(f, "  ")?;
    fmt_bytes_as_hex(f, bytes, &padding)?;
    write!(f, "  ")?;

    write!(f, "| ")?;
    fmt_bytes_as_char(f, cp, bytes, &padding)?;
    write!(f, " |")?;

    Ok(())
}

fn calculate_begin_padding(address_offset: usize, row_width: usize) -> usize {
    debug_assert!(row_width != 0,
                  "A zero row width is can not be used to calculate the begin padding");
    address_offset % row_width
}

fn calculate_end_padding(data_size: usize, row_width: usize) -> usize {
    debug_assert!(row_width != 0,
                  "A zero row width is can not be used to calculate the end padding");
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
        let mut offset = 0;
        let mut separator = "";

        if self.data.len() + begin_padding + end_padding <= self.row_width {
            return fmt_line(f,
                            address,
                            &self.codepage,
                            &self.data,
                            &Padding::new(begin_padding, end_padding));
        }

        if begin_padding != 0 {
            let slice = &self.data[offset..offset + self.row_width - begin_padding];
            fmt_line(f,
                     address,
                     &self.codepage,
                     &slice,
                     &Padding::from_left(begin_padding))?;
            offset += self.row_width - begin_padding;
            address += self.row_width;
            separator = "\n";
        }


        while offset + (self.row_width - 1) < self.data.len() {
            let slice = &self.data[offset..offset + self.row_width];
            write!(f, "{}", separator)?;
            fmt_line(f, address, &self.codepage, &slice, &Padding::default())?;
            offset += self.row_width;
            address += self.row_width;
            separator = "\n";
        }

        if end_padding != 0 {
            let slice = &self.data[offset..];
            write!(f, "{}", separator)?;
            fmt_line(f,
                     address,
                     &self.codepage,
                     &slice,
                     &Padding::from_right(end_padding))?;
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

        let row_view = HexViewBuilder::new(&data).row_width(data.len()).finish();

        let result = format!("{}", row_view);

        assert_eq!(result,
                   "00000000  40 41 42 43 44 45 46 47 48 49 4A 4B 4C 4D 4E 4F  | @ABCDEFGHIJKLMNO |");
    }

    #[test]
    fn an_incomplete_line_is_padded_on_the_right() {
        let data = ['a' as u8; 10];

        let row_view = HexViewBuilder::new(&data).row_width(16).finish();

        let result = format!("{}", row_view);

        assert_eq!(result,
                   "00000000  61 61 61 61 61 61 61 61 61 61                    | aaaaaaaaaa       |");
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

        assert_eq!(result,
                   "00000000                 61 61 61 61 61 61 61 61 61 61 61  |      aaaaaaaaaaa |");
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

        assert_eq!(result,
                   "00000000                 61 61 61 61 61 61 61 61           |      aaaaaaaa    |");
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

        let row_view = HexViewBuilder::new(&data).row_width(16).finish();

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

    #[test]
    fn all_characters_can_be_printed_in_codepage_hex() {
        let data: Vec<u8> = (0u16..256u16).map(|v| v as u8).collect();

        let dump_view = HexViewBuilder::new(&data)
            .codepage(&byte_mapping::CODEPAGE_HEX)
            .address_offset(20)
            .row_width(16)
            .finish();

        let result = format!("{}", dump_view);
        println!("{}", result);

        assert!(!result.is_empty());
        assert_eq!(result, r##"00000010              00 01 02 03 04 05 06 07 08 09 0A 0B  |     ............ |
00000020  0C 0D 0E 0F 10 11 12 13 14 15 16 17 18 19 1A 1B  | ................ |
00000030  1C 1D 1E 1F 20 21 22 23 24 25 26 27 28 29 2A 2B  | .... !"#$%&'()*+ |
00000040  2C 2D 2E 2F 30 31 32 33 34 35 36 37 38 39 3A 3B  | ,-./0123456789:; |
00000050  3C 3D 3E 3F 40 41 42 43 44 45 46 47 48 49 4A 4B  | <=>?@ABCDEFGHIJK |
00000060  4C 4D 4E 4F 50 51 52 53 54 55 56 57 58 59 5A 5B  | LMNOPQRSTUVWXYZ[ |
00000070  5C 5D 5E 5F 60 61 62 63 64 65 66 67 68 69 6A 6B  | \]^_`abcdefghijk |
00000080  6C 6D 6E 6F 70 71 72 73 74 75 76 77 78 79 7A 7B  | lmnopqrstuvwxyz{ |
00000090  7C 7D 7E 7F 80 81 82 83 84 85 86 87 88 89 8A 8B  | |}~............. |
000000A0  8C 8D 8E 8F 90 91 92 93 94 95 96 97 98 99 9A 9B  | ................ |
000000B0  9C 9D 9E 9F A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 AA AB  | ................ |
000000C0  AC AD AE AF B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 BA BB  | ................ |
000000D0  BC BD BE BF C0 C1 C2 C3 C4 C5 C6 C7 C8 C9 CA CB  | ................ |
000000E0  CC CD CE CF D0 D1 D2 D3 D4 D5 D6 D7 D8 D9 DA DB  | ................ |
000000F0  DC DD DE DF E0 E1 E2 E3 E4 E5 E6 E7 E8 E9 EA EB  | ................ |
00000100  EC ED EE EF F0 F1 F2 F3 F4 F5 F6 F7 F8 F9 FA FB  | ................ |
00000110  FC FD FE FF                                      | ....             |"##);
    }
}
