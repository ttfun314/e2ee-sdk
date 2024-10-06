use clap::{Arg, Command};
use e2ee_sdk::encyptor::generate_keys;

use std::fs::File;
use std::io::Write;

fn main() {
    let matches = Command::new("Key Generator")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Generates RSA key pairs")
        .arg(
            Arg::new("key_size")
                .short('s')
                .long("size")
                .help("Size of the key in bits")
                .value_name("KEY_SIZE")
                .required(true),
        )
        .arg(
            Arg::new("private_key")
                .short('p')
                .long("private")
                .help("File to save the private key")
                .value_name("PRIVATE_KEY_FILE")
                .required(true),
        )
        .arg(
            Arg::new("public_key")
                .short('u')
                .long("public")
                .help("File to save the public key")
                .value_name("PUBLIC_KEY_FILE")
                .required(true),
        )
        .get_matches();

    let key_size = matches
        .get_one::<String>("key_size")
        .expect("Invalid key size");
    let key_size = key_size.parse::<usize>().expect("Invalid key size");
    let private_key_file = matches.get_one::<String>("private_key").unwrap();
    let public_key_file = matches.get_one::<String>("public_key").unwrap();

    let (private_key, public_key) = generate_keys(key_size);

    let mut private_file =
        File::create(private_key_file).expect("Unable to create private key file");
    private_file
        .write_all(private_key.as_bytes())
        .expect("Unable to write private key");

    let mut public_file = File::create(public_key_file).expect("Unable to create public key file");
    public_file
        .write_all(public_key.as_bytes())
        .expect("Unable to write public key");

    println!(
        "Keys generated and saved to {} and {}",
        private_key_file, public_key_file
    );
}
