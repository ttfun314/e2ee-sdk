# E2EE SDK contains 3 parts:
# 1. End-to-End Encryption (E2EE)
The first part contains 3 core functions
The algorithm using here is RSA from rsa crate 
``` Rust
// Generate a pair of asymmetric keys (public and private).
pub fn generate_keys(key_size: usize) -> (String, String) {}

// Encrypt a given plaintext message using the public key.
pub fn encrypt(msg: &str, public_key: &str) -> String {}

// Encrypt a given plaintext message using the public key.
pub fn decrypt(cipher: &str, private_key_raw: &str) -> String {}
```

The first part comes with an CLI to test these functions

How to build it: `cargo build --release`
```
USAGE:
    ./target/release/e2ee-sdk [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    generate_keys    Generates RSA key pairs, save keys to files 
    encrypt          Encrypts a message using public key in the file specified by --public_key_file
    decrypt          Decrypts a message using private key in the file specified by --private_key_file
    help             Print this message or the help of the given subcommand(s)

SUBCOMMANDS:
    generate_keys    Generates RSA key pairs
        --size <KEY_SIZE>            Size of the key in bits
        --private <PRIVATE_KEY_FILE> File to save the private key
        --public <PUBLIC_KEY_FILE>   File to save the public key

    encrypt          Encrypts a message
        -m, --message <MESSAGE>          The plaintext message to encrypt
        -f, --public_key_file <PUBLIC_KEY_FILE>
                                         The public key file to use for encryption

    decrypt          Decrypts a message
        -c, --cipher <CIPHER_TEXT>       The ciphertext to decrypt
        -k, --private_key_file <PRIVATE_KEY_FILE>
                                         The private key file to use for decryption
```

To run unit test for 3 core functions, run `make test`

# 2. SDK Development
## Android
The Rust interface exposed to Java code is
``` Java
class Encryptor {
    static {
        System.loadLibrary("e2ee_sdk");
    }

    public native String encrypt(String msg, String publicKey);

    public native String decrypt(String cipher, String privateKey);
    public native String[] generateKeys(int keySize); 
}
```

To build the android library, run `make android`
The script would create a .aar file in /android/mylibrary/build/outputs/aar/mylibrary-release.aar
The interface of android library is

  <ul>
    <li>init(): Initializes the cryptographic keys and save them to encrypted shared preferences if they are not already present.</li>
    <li>getPublicKey: Retrieves the stored public key.</li>
    <li>encrypt(String, String): Encrypts a message using the provided public key.</li>
    <li>decrypt(String): Decrypts a cipher text using the stored private key.</li>
  </ul>

``` Java
 CryptoSDK cryptoSDK = new CryptoSDK(context);
 cryptoSDK.init();
 String publicKey = cryptoSDK.getPublicKey();
 String encryptedMessage = cryptoSDK.encrypt("Hello, World!", publicKey);
 String decryptedMessage = cryptoSDK.decrypt(encryptedMessage);
```

Steps to use the android lib in your project:
- Copy the .aar file to your project at /app/libs (create libs directory if not exist)
- Add following snippet to your app/build.gradle.kts or app/build.gradle depending on your project

```
    implementation(files("libs/mylibrary-release.aar"))
    implementation("androidx.security:security-crypto:1.1.0-alpha04")
```
- `import com.x.e2ee.CryptoSDK` to start using the sdk

Example testcase using the sdk

``` Kotlin
@RunWith(AndroidJUnit4::class)
class EncryptorTest {

    @Test
    fun testEncrypt() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val cryptoSDK = CryptoSDK(context)
        cryptoSDK.init()
        val message = "Hello, Android!"
        val encryptedMessage = cryptoSDK.encrypt(message, cryptoSDK.publicKey)

        // Assert that the encrypted message is not null and not equal to the original message
        assertNotNull(encryptedMessage)
        assertNotEquals(message, encryptedMessage)

        Log.d("EncryptorTest", "Encrypted message: $encryptedMessage")

        val decryptedMessage = cryptoSDK.decrypt(encryptedMessage)
        Log.d("EncryptorTest", "Decrypted message: $decryptedMessage")
        assertEquals(message, decryptedMessage)
    }

}
```

Link to the example project [e2ee-sdk-android-example](https://github.com/ttfun314/e2ee-sdk-android-example/blob/main/app/src/androidTest/java/com/example/myapplicationusingnewlib/EncryptorTest.kt#L18)

## iOS (Using swift)
How to build target: `make ios`
Steps to use the ios lib in your project:
- Import file from /target/x86_64-apple-ios/release/libe2ee_sdk.a and file e2ee_sdk.h to iOS project
- Create a Bridging-Header.m file 
- Config build path to include those file during the build
- `import Foundation` and start using the lib

``` Swift
func encryptMessage(msg: String, publicKey: String) -> String? {
        let cMsg = msg.cString(using: .utf8)
        let cPublicKey = publicKey.cString(using: .utf8)

        if let encryptedCStr = encrypt_ffi(cMsg, cPublicKey) {
            let encryptedMessage = String(cString: encryptedCStr)
            free_c_string(UnsafeMutablePointer(mutating: encryptedCStr)) // Free the C string memory
            return encryptedMessage
        }
        return nil
}
```

Link to example project [e2ee-sdk-ios-example](https://github.com/ttfun314/e2ee-sdk-ios-example/blob/main/Hello%20WorldTests/Hello_WorldTests.swift#L47)
