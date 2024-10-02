use std::io::{self, Read, Write};

use base64::prelude::*;
use clap::Parser;
use clio::Input;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
#[clap(rename_all = "lower")]
enum Alphabet {
    Standard,
    UrlSafe,
}

/// Base64 encoder/decoder that can handle any of the common Base64 variants.
/// When decoding, it will try all variants.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// File to encode/decode, default value means stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,

    /// Enable decode mode (default is encode mode)
    #[clap(long, short, default_value = "false")]
    decode: bool, // would use an enum normally but we want -d/-D to behave the same as the ordinary base64 tool

    /// Base64 alphabet to use (ignored when decoding)
    #[clap(long, short, default_value = "urlsafe")]
    alphabet: Alphabet,

    /// Enables padding (ignored when decoding)
    #[clap(long, short, default_value = "false")]
    padding: bool,
}

impl Arguments {
    fn engine(&self) -> base64::engine::GeneralPurpose {
        match (self.alphabet, self.padding) {
            (Alphabet::Standard, true) => BASE64_STANDARD,
            (Alphabet::Standard, false) => BASE64_STANDARD_NO_PAD,
            (Alphabet::UrlSafe, true) => BASE64_URL_SAFE,
            (Alphabet::UrlSafe, false) => BASE64_URL_SAFE_NO_PAD,
        }
    }
}

/// Try decoding the input against any of the four Base64 variants.
fn try_decode(input: impl AsRef<[u8]>) -> Option<Vec<u8>> {
    let input = input.as_ref();
    [
        BASE64_STANDARD,
        BASE64_STANDARD_NO_PAD,
        BASE64_URL_SAFE,
        BASE64_URL_SAFE_NO_PAD,
    ]
    .into_iter()
    .filter_map(|alphabet| alphabet.decode(input).ok())
    .next()
}

fn main() {
    let mut arguments = Arguments::parse();

    let mut input: Vec<u8> = vec![];
    arguments
        .input
        .read_to_end(&mut input)
        .expect("failed to read input");
    let input = input.trim_ascii_end();

    if arguments.decode {
        let decoded_bytes = try_decode(input)
            .expect("Base64 decoding failed for each supported variant (standard/URL-safe; with or without padding)");
        io::stdout()
            .write_all(&decoded_bytes)
            .expect("failed to write to stdout");
    } else {
        println!("{}", arguments.engine().encode(&input));
    }
}
