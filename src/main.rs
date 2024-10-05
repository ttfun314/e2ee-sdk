use e2ee_sdk::encyptor::{decrypt, encrypt};

fn main() {
    let public_key = include_str!("../keys/public_key.pem");
    let private_key = include_str!("../keys/private_key.pem");

    let msg = "Hello, world!";
    let cipher = encrypt(msg, public_key);
    let decrypted = decrypt(&cipher, private_key);
    println!("Original: {}", msg);
    println!("Cipher: {}", cipher);
    println!("Decrypted: {}", decrypted);
}
