use base32;
use hex;
use std::env;
use stellar_strkey::Strkey;

enum DecodeType {
    XdrToHash,
    HashToXdr,
}

fn hash_to_xdr(input: &String) {
    let decoded = Strkey::Contract(stellar_strkey::Contract(
        hex::decode(input).unwrap().as_slice().try_into().unwrap(),
    ))
    .to_string();

    println!("{}", decoded);
}

fn xdr_to_hash(input: &String) {
    let mut data = base32::decode(base32::Alphabet::RFC4648 { padding: false }, input)
        .expect("Failed decoding base32");

    // Remove the first element
    data.remove(0);

    // Remove the two last elements
    let len = data.len();
    data.truncate(len - 2);

    println!("{}", hex::encode(&data));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <xdr-hash/hash-xdr> <string to decode>", args[0]);
        std::process::exit(1);
    }

    let decode_type = match args[1].as_str() {
        "xdr-hash" => DecodeType::XdrToHash,
        "hash-xdr" => DecodeType::HashToXdr,
        _ => {
            eprintln!("Usage: {} <xdr-hash/hash-xdr> <string to decode>", args[0]);
            std::process::exit(1);
        }
    };

    let input = &args[2];

    match decode_type {
        DecodeType::XdrToHash => xdr_to_hash(input),
        DecodeType::HashToXdr => hash_to_xdr(input),
    }
}
