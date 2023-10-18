/*

To run:

cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:

cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10

*/

use caeser_cipher_cli::{decrypt, encrypt, fixed_encrypt, fixed_decrypt};
use clap::Parser;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// The message to encrypt or decrypt
    #[arg(short, long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,

    /// Perform fixed encryption
    #[arg(short, long)]
    fixed_encrypt: bool,

    /// Perform fixed decryption
    #[arg(short, long)]
    numfixed_decrypt: bool,
}

// run it
fn main() {
    let args = Args::parse();

    if args.encrypt {
        println!("{}", encrypt(&args.message, args.shift));
    } else if args.decrypt {
        println!("{}", decrypt(&args.message, args.shift));
    } else if args.fixed_encrypt {
        println!("{}", fixed_encrypt(&args.message));
    } else if args.numfixed_decrypt {
        println!("{}", fixed_decrypt(&args.message));
    } else {
        println!("Please specify an action: --encrypt, --decrypt, --fixed-encrypt, or --fixed-decrypt");
    }
}
