use rc4ok::RC4ok;

use std::env;
use std::io::{self, Write};

fn print_usage() {
    println!("Usage: rc4ok [OPTIONS] SEED...");
    println!("Constructs a pseudo-random number generator, based on improved RC4 stream cipher, ");
    println!("from the given input seed and yields random output.\n");
    println!("Options:");
    println!("\t-h                \tPrint help text.");
    println!("\t-b NUMBER-OF-BYTES\tOutput `n` pseudo-random bytes onto STDOUT.");
    println!(
        "\t-s                \tOutput a continuous stream of pseudo-random bytes onto STDOUT."
    );
}

enum OutputType {
    RequestedMany,
    Stream,
}

fn main() {
    let mut arg_iter = env::args().skip(1);
    let arg_cnt = arg_iter.len();
    if arg_cnt < 1 {
        print_usage();
        return;
    }

    let mut out_type = OutputType::RequestedMany;
    let mut requested_bytes = 0usize;

    let option = arg_iter.next().unwrap();
    match option.as_str() {
        "-h" => {
            print_usage();
            return;
        }
        "-s" => {
            out_type = OutputType::Stream;
        }
        "-b" => {
            requested_bytes = arg_iter
                .next()
                .expect("Requested number of bytes must be a valid integer.")
                .parse::<usize>()
                .expect("Failed to parse requested number of bytes.");
        }
        o @ _ => {
            eprintln!("Error: unrecognised option `{}`, try passing `-h`.", o);
            return;
        }
    }

    let seed = arg_iter
        .next()
        .expect("Must provide with a non-empty SEED string.");
    let mut rc4ok_rng = RC4ok::init(seed.as_bytes());

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match out_type {
        OutputType::RequestedMany => {
            let mut off = 0;
            let mut buf = [0u8; 1024];

            while off < requested_bytes {
                let rm_bytes = requested_bytes - off;
                let gen_bytes = if rm_bytes < buf.len() {
                    rm_bytes
                } else {
                    buf.len()
                };

                rc4ok_rng.generate(&mut buf[..gen_bytes]);

                handle.write_all(&buf[..gen_bytes]).unwrap();
                off += gen_bytes;
            }
        }
        OutputType::Stream => {
            let mut buf = [0u8; 1024];

            loop {
                rc4ok_rng.generate(&mut buf);
                handle.write_all(&buf).unwrap();
            }
        }
    }
}
