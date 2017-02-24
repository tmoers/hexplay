#![feature(test)]

extern crate hexplay;
extern crate test;

use test::Bencher;
use hexplay::*;

#[bench]
fn bench_format_one_complete_line(b: &mut Bencher) {
    let data: Vec<u8> = (0x40..0x40 + 0xF + 1).collect();

    let row_view = HexViewBuilder::new(&data)
        .row_width(data.len())
        .finish();

    b.iter(|| format!("{}", row_view));
}

#[bench]
fn bench_format_one_unaligned_incomplete_line(b: &mut Bencher) {
    let data: Vec<u8> = (0x45..0x45 + 8 + 1).collect();

    let row_view = HexViewBuilder::new(&data)
        .address_offset(5)
        .row_width(16)
        .finish();

    b.iter(|| format!("{}", row_view));
}

#[bench]
fn bench_format_a_medium_block_of_data(b: &mut Bencher) {
    let data: Vec<u8> = (0u16..256u16).map(|v| v as u8).collect();

    let row_view = HexViewBuilder::new(&data)
        .finish();

    b.iter(|| format!("{}", row_view));
}

#[bench]
fn bench_format_a_big_block_of_data(b: &mut Bencher) {
    let data: Vec<u8> = (0u16..10 * 1024u16).map(|v| (v % 256) as u8).collect();

    let row_view = HexViewBuilder::new(&data)
        .finish();

    b.iter(|| format!("{}", row_view));
}
