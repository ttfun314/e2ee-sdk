pub mod encyptor;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use jni::{
    objects::{JClass, JString},
    sys::{jobjectArray, jstring},
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
pub extern "C" fn Java_com_x_e2ee_Encryptor_encrypt(
    env: JNIEnv,
    class: JClass,
    msg: JString,
    public_key: JString,
) -> jstring {
    Java_Encryptor_encrypt(env, class, msg, public_key)
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

#[no_mangle]
pub extern "C" fn Java_com_x_e2ee_Encryptor_decrypt(
    env: JNIEnv,
    class: JClass,
    cipher: JString,
    private_key: JString,
) -> jstring {
    Java_Encryptor_decrypt(env, class, cipher, private_key)
}

#[no_mangle]
pub extern "C" fn Java_Encryptor_generateKeys(
    env: JNIEnv,
    _class: JClass,
    key_size: i32,
) -> jobjectArray {
    // Convert Java strings (JString) to Rust strings
    let key_size = key_size as usize;
    // Call Rust key generation logic
    let (private_key, public_key) = encyptor::generate_keys(key_size as usize);
    // Convert Rust strings to JNI JString
    let private_key_jstring = env
        .new_string(private_key)
        .expect("Couldn't create private key string!");
    let public_key_jstring = env
        .new_string(public_key)
        .expect("Couldn't create public key string!");

    // Create a new Java String array with a length of 2
    let string_array = env
        .new_object_array(
            2,
            env.find_class("java/lang/String").unwrap(),
            env.new_string("").unwrap(),
        )
        .unwrap();

    // Set the public and private key strings into the array
    env.set_object_array_element(string_array, 0, private_key_jstring)
        .unwrap();
    env.set_object_array_element(string_array, 1, public_key_jstring)
        .unwrap();

    // Return the array
    string_array
}

#[no_mangle]
pub extern "C" fn Java_com_x_e2ee_Encryptor_generateKeys(
    env: JNIEnv,
    class: JClass,
    key_size: i32,
) -> jobjectArray {
    Java_Encryptor_generateKeys(env, class, key_size)
}

// FFI functions

#[repr(C)]
pub struct RSAKeyPair {
    pub private_key: *const c_char,
    pub public_key: *const c_char,
}

#[no_mangle]
pub extern "C" fn generate_keys_ffi(key_size: usize) -> RSAKeyPair {
    let (private_key, public_key) = encyptor::generate_keys(key_size);

    let private_key_c = CString::new(private_key).unwrap();
    let public_key_c = CString::new(public_key).unwrap();
    RSAKeyPair {
        private_key: private_key_c.into_raw(),
        public_key: public_key_c.into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn encrypt_ffi(msg: *const c_char, public_key: *const c_char) -> *const c_char {
    let msg = unsafe {
        assert!(!msg.is_null());
        CStr::from_ptr(msg).to_str().unwrap()
    };

    let public_key = unsafe {
        assert!(!public_key.is_null());
        CStr::from_ptr(public_key).to_str().unwrap()
    };

    let encrypted_msg = encyptor::encrypt(msg, public_key);
    CString::new(encrypted_msg).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn decrypt_ffi(cipher: *const c_char, private_key: *const c_char) -> *const c_char {
    let cipher = unsafe {
        assert!(!cipher.is_null());
        CStr::from_ptr(cipher).to_str().unwrap()
    };

    let private_key = unsafe {
        assert!(!private_key.is_null());
        CStr::from_ptr(private_key).to_str().unwrap()
    };

    let decrypted_msg = encyptor::decrypt(cipher, private_key);
    CString::new(decrypted_msg).unwrap().into_raw()
}

// A helper function to free the C string when done to avoid memory leaks.
#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s); // Automatically deallocates the memory
    }
}
