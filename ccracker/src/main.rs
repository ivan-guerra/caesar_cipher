use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long, help = "file containing ciphertext")]
    ciphertext_file: Option<std::path::PathBuf>,

    #[arg(
        short = 'a',
        long,
        value_enum,
        default_value_t = ccracker::Attack::Dictionary,
        help = "attack type"
    )]
    attack: ccracker::Attack,
}

fn main() {
    let args = Args::parse();
    let config = ccracker::Config::new(args.ciphertext_file, args.attack);

    if let Err(e) = ccracker::run(&config) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
