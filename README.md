Implements rust's Display trait to format a u8 slice as many hex editors
do. This might be useful for dumping a binary blob for debugging purposes.

[![Build Status][tr-img]][tr]


### Documentation

The API documentation can be found here:
[https://docs.rs/crate/hexplay/](https://docs.rs/crate/hexplay/).


### Example

Here's an example that prints a hex view of a slice of some vector's data:

```rust
extern crate hexplay;

use hexplay::HexViewBuilder;

fn main() {
    // The buffer we want to display
    let data : Vec<u8> = (0u8..200u8).collect();

    // Build a new HexView using the provider builder
    let view = HexViewBuilder::new(&data[40..72])
        .address_offset(40)
        .row_width(16)
        .finish();

    println!("{}", view);
}
```

This will result in the following output:

```text
00000020                          28 29 2A 2B 2C 2D 2E 2F  |         ()*+,-./ |
00000030  30 31 32 33 34 35 36 37 38 39 3A 3B 3C 3D 3E 3F  | 0123456789:;<=>? |
00000040  40 41 42 43 44 45 46 47                          | @ABCDEFG         |
```

### Installation

`hexplay` is on [crates.io][crates], so you can include it in your project
like so:

```toml
[dependencies]
hexplay = "*"
```

Because this crate uses the `?` operator, you need [rust v1.13.0][rust-v13]
or higher.


### License

Hexplay is licensed under the terms of the [MIT license][mit].



[crates]:    https://crates.io/crates/hexplay
[mit]:       https://en.wikipedia.org/wiki/MIT_License
[rust-v13]:  https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1130-2016-11-10
[tr-img]:    https://travis-ci.org/tmoers/hexplay.svg?branch=master
[tr]:        https://travis-ci.org/tmoers/hexplay
