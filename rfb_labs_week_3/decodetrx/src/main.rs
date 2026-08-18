use clap::{Arg, Command};
use decodetrx::decode_transaction;

fn main() {
    // ── Build the CLI definition ───────────────────────────────────────────────
    // clap's builder API: we describe every argument here, then parse argv.
    let matches = Command::new("decodetrx")
        .version("1.0")
        .about("Bitcoin Transaction Decoder — parses a raw hex transaction into JSON")
        // One required positional argument: the raw transaction hex string
        .arg(
            Arg::new("transaction_hex")
                .help("Raw transaction hex string (required)")
                .required(true)
                .index(1),   // positional: first argument after the binary name
        )
        .get_matches();  // parse std::env::args()

    // ── Retrieve the argument value ────────────────────────────────────────────
    // get_one::<String> returns an Option<&String>; unwrap is safe because the
    // argument is marked required above — clap will have already exited with an
    // error message if it was missing.
    let transaction_hex = matches
        .get_one::<String>("transaction_hex")
        .unwrap()
        .to_string();

    // ── Call the decoder from the library ─────────────────────────────────────
    match decode_transaction(transaction_hex) {
        Ok(json) => {
            // Pretty-printed JSON to stdout
            println!("{}", json);
        }
        Err(e) => {
            // Print errors to stderr and exit with a non-zero status code
            eprintln!("Error decoding transaction: {}", e);
            std::process::exit(1);
        }
    }
}

// Example usage:
// cargo run -- 0200000000010196277c04c986c1ad78c909287fd12dba2924324699a0232e0533f46a6a3916bb0100000000ffffffff026400000000000000160014274ae586ad2035efb4c25049c155f98310d7e106ca16440000000000160014599bcef6387256c6b019030c421b4a4d382fe2600247304402204d94a1e4047ca38a450177ccb6f88585ca147f1939df343d8ac5d962c5f35bb302206f7fa42c21c47ebccdc460393d35c5dfd3b6f0a26cf10fac23d3e6fab71835c20121020cb972a66e3fb1cdcc9efcad060b4457ebec534942700d4af1c0d82a33aa13f100000000
