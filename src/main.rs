use clap::{App, Arg};
use rug::Float;
use std::fs;
use std::io::Write;

const MAGIC_HEADER: &str = "constore\x1F";

fn run() {
    let mut cli = App::new("Constore")
        .arg(
            Arg::with_name("encode")
                .short("e")
                .long("encode")
                .value_name("FILE")
                .help("File to encode")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .value_name("FILE")
                .help("File to decode")
                .takes_value(true),
        );
    let matches = cli.clone().get_matches();

    let (encode_vec, decode_vec) = compute_byte_vectors();

    if let Some(encode_file_path) = matches.value_of("encode") {
        encode_file(String::from(encode_file_path), &encode_vec);
    } else if let Some(decode_file_path) = matches.value_of("decode") {
        decode_file(String::from(decode_file_path), &decode_vec);
    } else {
        cli.print_help().unwrap();
    }
}

fn main() {
    run();
}

fn encode_file(input_file_path: String, encode_vec: &Vec<u8>) {
    let input_bytes = fs::read(&input_file_path).unwrap();

    let mut output_file = fs::File::create(&format!("{}.constore", input_file_path)).unwrap();

    output_file
        .write_all(&format!("{}{: >32}", MAGIC_HEADER, "sqrt(157)").into_bytes())
        .unwrap();

    let mut output_vec: Vec<u8> = vec![];

    for (i, byte) in input_bytes.iter().enumerate() {
        if i % 1_000_000 == 0 {
            eprintln!("encoded {: >3}mb", i / 1_000_000);
        }
        output_vec.push(code_byte(*byte, &encode_vec));
    }

    output_file.write_all(&output_vec).unwrap();
}

fn decode_file(input_file_path: String, decode_vec: &Vec<u8>) {
    let input_bytes = fs::read(&input_file_path).unwrap();

    if !input_bytes.starts_with(MAGIC_HEADER.as_bytes()) {
        panic!("Invalid constore file");
    }

    let input_bytes = input_bytes.split_at(41).1;

    let output_file_path = if input_file_path.ends_with(".constore") {
        input_file_path
            .split_at(input_file_path.len() - ".constore".len())
            .0
    } else {
        input_file_path.as_str()
    };

    let mut output_file = fs::File::create(output_file_path).unwrap();
    let mut output_vec: Vec<u8> = vec![];

    for (i, byte) in input_bytes.iter().enumerate() {
        if i % 1_000_000 == 0 {
            eprintln!("decoded {: >3}mb", i / 1_000_000);
        }
        output_vec.push(code_byte(*byte, &decode_vec));
    }

    output_file.write_all(&output_vec).unwrap();
}

fn compute_byte_vectors() -> (Vec<u8>, Vec<u8>) {
    let n = 157.0;
    let chunk_size = 4;

    let sq = Float::with_val(20, n).sqrt().to_string_radix(2, None);

    let digits = String::from(sq.split('.').nth(1).unwrap().split('e').nth(0).unwrap());

    // println!("{} {}", digits, n);

    let mut encode_chunks: Vec<u8> = vec![0; 16];
    let mut decode_chunks: Vec<u8> = vec![];

    for i in 0..=(digits.len() - chunk_size) {
        let chunk = &digits[i..(i + chunk_size)];
        let nib = u8::from_str_radix(chunk, 2).unwrap();

        // println!("{} {:0>8b}", chunk, nib);
        // assert!(chunk == format!("{:0>8b}", nib));

        decode_chunks.push(nib);

        // if chunks.len() >= max_set_size {
        //     // println!("{:<10} {}", n, i + chunk_size);
        //     println!("{:<10} {}", n, digits);
        //     break;
        // }
    }

    for (i, nib) in decode_chunks.iter().enumerate() {
        encode_chunks[*nib as usize] = i as u8;
    }

    (encode_chunks, decode_chunks)
}

fn code_byte(byte: u8, code_vec: &Vec<u8>) -> u8 {
    let nibble = byte & 15;
    let out_byte = code_vec[nibble as usize];
    let nibble = (byte & (15 << 4)) >> 4;
    let out_byte = out_byte | (code_vec[nibble as usize] << 4);

    out_byte
}
