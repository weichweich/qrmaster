use bardecoder;
use clap::{App, Arg, SubCommand};
use image::Luma;
use qrcode::QrCode;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use stderrlog;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const SUB_CMD_ENCODE: &str = "encode";
const SUB_CMD_DECODE: &str = "decode";

const ARG_IN_FILE: &str = "in";
const ARG_OUT_FILE: &str = "out";
const ARG_VERBOSE: &str = "verbose";

fn main() {
    let matches = App::new("QR Master")
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name(ARG_VERBOSE)
                .short("v")
                .help("Sets the level of verbosity")
                .multiple(true)
                .takes_value(false),
        )
        .subcommand(
            SubCommand::with_name(SUB_CMD_ENCODE)
                .about("Encodes the given data into an qr code")
                .arg(
                    Arg::with_name(ARG_IN_FILE)
                        .short("i")
                        .help("Path of the input file")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(ARG_OUT_FILE)
                        .short("o")
                        .help("Path where the output image will be written")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(SUB_CMD_DECODE)
                .about("Decodes a QR code")
                .arg(
                    Arg::with_name(ARG_IN_FILE)
                        .short("i")
                        .help("Path of the input image file")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(ARG_OUT_FILE)
                        .short("o")
                        .help("Path where the output will be written")
                        .takes_value(true),
                ),
        )
        .get_matches();
    
    stderrlog::new().verbosity(matches.occurrences_of(ARG_VERBOSE) as usize).init().unwrap();

    match matches.subcommand() {
        (SUB_CMD_DECODE, Some(sub_match)) => {
            let path = sub_match.value_of(ARG_IN_FILE).unwrap();
            let img = image::open(path).unwrap();

            let decoder = bardecoder::default_decoder();
            let results = decoder.decode(&img);
            if let Some(out) = sub_match.value_of(ARG_OUT_FILE) {
                if !fs::metadata(out).is_ok() {
                    panic!("Path exits")
                }
                let mut file = File::create("foo.txt").unwrap();
                for result in results {
                    let content = result.unwrap();
                    file.write_all(&content).unwrap();
                }
            } else {
                for result in results {
                    io::stdout().write_all(&result.unwrap()).unwrap();
                }
            }
        }
        (SUB_CMD_ENCODE, Some(sub_match)) => {
            let mut payload = Vec::new();
            if let Some(input) = sub_match.value_of(ARG_IN_FILE) {
                let mut file = File::open(input).unwrap();
                file.read_to_end(&mut payload).unwrap();
            } else {
                io::stdin().read_to_end(&mut payload).unwrap();
            }
            // Encode some data into bits.
            let code = QrCode::new(payload).unwrap();
            // Render the bits into an image.
            let image = code.render::<Luma<u8>>().build();
            // Save the image.
            let out = sub_match.value_of(ARG_OUT_FILE).unwrap();
            image.save(out).unwrap();
        }
        _ => println!("Unknown subcommand"),
    }
}
