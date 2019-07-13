use bimap::BiMap;
use clap::{App, Arg};
use rug::Float;
use std::fs;

fn run() {
    let cli = App::new("Constore")
        .arg(Arg::with_name("file").index(1))
        .get_matches();

    let byte_map = compute_byte_map();

    let input_file_path = cli.value_of("file").unwrap();

    let input_bytes = fs::read(input_file_path).unwrap();

    let output_file = fs::File::create(&format!("{}.constore", input_file_path)).unwrap();

    for byte in input_bytes {
        // write those bytes!
    }
}

fn main() {
    run();
}

fn compute_byte_map() -> BiMap<u8, u8> {
    let n = 157.0;
    let chunk_size = 4;

    let sq = Float::with_val(20, n).sqrt().to_string_radix(2, None);

    let digits = String::from(sq.split('.').nth(1).unwrap().split('e').nth(0).unwrap());

    // println!("{} {}", digits, n);

    let mut chunks = BiMap::new();

    for i in 0..=(digits.len() - chunk_size) {
        let chunk = &digits[i..(i + chunk_size)];
        let nib = u8::from_str_radix(chunk, 2).unwrap();

        // println!("{} {:0>8b}", chunk, nib);
        // assert!(chunk == format!("{:0>8b}", nib));

        chunks.insert(i as u8, nib);

        // if chunks.len() >= max_set_size {
        //     // println!("{:<10} {}", n, i + chunk_size);
        //     println!("{:<10} {}", n, digits);
        //     break;
        // }
    }
    chunks
}

fn encode_byte(byte: u8, map: &BiMap<u8, u8>) -> u8 {
    let nibble = byte & 7;
    let out_byte = map.get_by_right(&nibble).unwrap();
    let nibble = (byte & (7 << 4)) >> 4;
    let out_byte = out_byte | (*map.get_by_right(&nibble).unwrap() << 4);

    out_byte
}
