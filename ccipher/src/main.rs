use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "encryption/decryption key")]
    key: i32,

    #[arg(short = 'i', long, help = "input plaintext/ciphertext file")]
    input_file: Option<std::path::PathBuf>,

    #[arg(short = 'o', long, help = "output plaintext/ciphertext file")]
    output_file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    let config = ccipher::Config::new(args.key, args.input_file, args.output_file);

    if let Err(e) = ccipher::run(&config) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
