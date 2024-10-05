pub mod encyptor;

use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};

#[no_mangle]
pub extern "C" fn Java_Encryptor_encrypt(
    env: JNIEnv,         // JNIEnv pointer, always the first argument in JNI
    _class: JClass,      // Class or object reference (unused in this case)
    msg: JString,        // The message to encrypt (Java String)
    public_key: JString, // The public key (Java String)
) -> jstring {
    // Convert Java strings (JString) to Rust strings
    let msg: String = env
        .get_string(msg)
        .expect("Couldn't get message string!")
        .into();
    let public_key: String = env
        .get_string(public_key)
        .expect("Couldn't get public key string!")
        .into();

    // Call Rust encryption logic
    let encrypted = encyptor::encrypt(&msg, &public_key);

    // Convert Rust string back to Java string (JString)
    let output = env
        .new_string(encrypted)
        .expect("Couldn't create output string!");
    output.into_inner() // Return a Java string (jstring)
}

#[no_mangle]
pub extern "C" fn Java_Encryptor_decrypt(
    env: JNIEnv,          // JNIEnv pointer, always the first argument in JNI
    _class: JClass,       // Class or object reference (unused in this case)
    cipher: JString,      // The cipher text to decrypt (Java String)
    private_key: JString, // The private key (Java String)
) -> jstring {
    // Convert Java strings (JString) to Rust strings
    let cipher: String = env
        .get_string(cipher)
        .expect("Couldn't get cipher string!")
        .into();
    let private_key: String = env
        .get_string(private_key)
        .expect("Couldn't get private key string!")
        .into();

    // Call Rust decryption logic
    let decrypted = encyptor::decrypt(&cipher, &private_key);

    // Convert Rust string back to Java string (JString)
    let output = env
        .new_string(decrypted)
        .expect("Couldn't create output string!");
    output.into_inner() // Return a Java string (jstring)
}
