//! Render NEPO source to SVG from the command line.
//!
//! Used to produce the candidate SVGs for `examples/nepo/comparisons`, and
//! handy when checking geometry without going through Typst.
//!
//!   cargo run --bin nepo-render -- '{"code":"Start","language":"de"}'
//!   cargo run --bin nepo-render -- --file request.json

use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let input = match args.first().map(String::as_str) {
        Some("--file") => {
            let path = args.get(1).expect("--file needs a path");
            std::fs::read_to_string(path).expect("read request")
        }
        Some(json) => json.to_string(),
        None => {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .expect("read stdin");
            buffer
        }
    };

    match nepo_wasm::render_request(&input) {
        Ok(svg) => println!("{svg}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
