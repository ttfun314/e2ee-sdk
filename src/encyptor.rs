use rsa::{
    pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPrivateKey,
    RsaPublicKey,
};
pub fn encrypt(msg: &str, public_key: &str) -> String {
    let public_key =
        RsaPublicKey::from_public_key_pem(&public_key).expect("failed to parse public key");
    let mut rng = rand::thread_rng();

    let enc_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes())
        .expect("failed to encrypt");
    hex::encode(enc_data)
}

pub fn decrypt(cipher: &str, private_key_raw: &str) -> String {
    let private_key =
        RsaPrivateKey::from_pkcs1_pem(private_key_raw).expect("failed to parse private key");

    let enc_data = hex::decode(cipher).expect("failed to decode hex");
    let dec_data = private_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");

    String::from_utf8(dec_data).expect("failed to convert to string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let public_key = include_str!("../keys/public_key.pem");
        let private_key = include_str!("../keys/private_key.pem");

        let msg = "Hello, world!";
        let cipher = encrypt(msg, public_key);
        let decrypted = decrypt(&cipher, private_key);

        assert_ne!(msg, cipher);
        assert_eq!(msg, decrypted);
    }
}
