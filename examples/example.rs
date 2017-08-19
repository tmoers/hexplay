extern crate hexplay;

use hexplay::*;

fn main() {
    let data: Vec<u8> = (0u16..256u16).map(|v| v as u8).collect();

    let default_view = HexViewBuilder::new(&data)
        .finish();
    println!("Default view of all data:\n{}\n\n", default_view);

    let ascii_view = HexViewBuilder::new(&data)
        .codepage(CODEPAGE_ASCII)
        .finish();
    println!("Ascii codepage view of all data:\n{}\n\n", ascii_view);

    let partial_view = HexViewBuilder::new(&data[10..80])
        .address_offset(10)
        .finish();
    println!("Default view of a subslice:\n{}\n\n", partial_view);

    let narrowed_view = HexViewBuilder::new(&data)
        .row_width(10)
        .finish();
    println!("Narrowed view of all data:\n{}\n\n", narrowed_view);

    let combined_view = HexViewBuilder::new(&data[10..180])
        .address_offset(10)
        .codepage(CODEPAGE_1252)
        .row_width(14)
        .replacement_character(std::char::REPLACEMENT_CHARACTER)
        .finish();
    println!("Custom view: \n{}\n\n", combined_view);

    let color_view = HexViewBuilder::new(&data)
        .force_color()
        .add_colors(vec![
            (hexplay::color::red(), 42..72),
            (hexplay::color::yellow_bold(), 10..11),
            (hexplay::color::green(), 32..38),
            (hexplay::color::blue(), 200..226),
        ])
        .finish();
    println!("Coloured view: \n");
    color_view.print().unwrap();
    println!("\n\n");
}
