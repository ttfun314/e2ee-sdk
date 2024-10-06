use clap::{Arg, Command};
use e2ee_sdk::encyptor::{decrypt, encrypt, generate_keys};

use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let matches = Command::new("Key Generator and Encryptor")
        .version("1.0")
        .author("ttfun314")
        .about("Generates RSA key pairs and encrypts messages")
        .subcommand(
            Command::new("generate_keys")
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
                ),
        )
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts a message")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .help("The plaintext message to encrypt")
                        .value_name("MESSAGE")
                        .required(true),
                )
                .arg(
                    Arg::new("public_key_file")
                        .short('f')
                        .long("public_key_file")
                        .help("The public key file to use for encryption")
                        .value_name("PUBLIC_KEY_FILE")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts a message")
                .arg(
                    Arg::new("cipher_text")
                        .short('c')
                        .long("cipher")
                        .help("The ciphertext to decrypt")
                        .required(true),
                )
                .arg(
                    Arg::new("private_key_file")
                        .short('k')
                        .long("private_key_file")
                        .help("The private key file to use for decryption")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate_keys") {
        let key_size = matches
            .get_one::<String>("key_size")
            .expect("key_size is required")
            .parse::<usize>()
            .expect("Invalid key size");
        let private_key_file = matches
            .get_one::<String>("private_key")
            .expect("private_key is required");
        let public_key_file = matches
            .get_one::<String>("public_key")
            .expect("public_key is required");

        let (private_key, public_key) = generate_keys(key_size);

        let mut private_file =
            File::create(private_key_file).expect("Unable to create private key file");
        private_file
            .write_all(private_key.as_bytes())
            .expect("Unable to write private key");

        let mut public_file =
            File::create(public_key_file).expect("Unable to create public key file");
        public_file
            .write_all(public_key.as_bytes())
            .expect("Unable to write public key");

        println!(
            "Keys generated and saved to {} and {}",
            private_key_file, public_key_file
        );
    } else if let Some(matches) = matches.subcommand_matches("encrypt") {
        let message = matches
            .get_one::<String>("message")
            .expect("message is required");
        let public_key_file = matches
            .get_one::<String>("public_key_file")
            .expect("public_key_file is required");

        let mut public_key = String::new();
        File::open(public_key_file)
            .expect("Unable to open public key file")
            .read_to_string(&mut public_key)
            .expect("Unable to read public key file");

        let encrypted_message = encrypt(message, &public_key);
        println!("{}", encrypted_message);
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let cipher_text = matches
            .get_one::<String>("cipher_text")
            .expect("cipher_text is required");
        let private_key_file = matches
            .get_one::<String>("private_key_file")
            .expect("private_key_file is required");

        let mut private_key = String::new();
        File::open(private_key_file)
            .expect("Unable to open private key file")
            .read_to_string(&mut private_key)
            .expect("Unable to read private key file");

        let decrypted_message = decrypt(cipher_text, &private_key);
        println!("Decrypted message: {}", decrypted_message);
    }
}
