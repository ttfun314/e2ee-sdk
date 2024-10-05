import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

class Encryptor {
    static {
        System.loadLibrary("e2ee_sdk");
    }

    public native String encrypt(String msg, String publicKey);

    public native String decrypt(String cipher, String privateKey);
    
    public static void main(String[] args) {
        Encryptor encryptor = new Encryptor();

        String publicKeyPath = "../keys/public_key.pem";
        String privateKeyPath = "../keys/private_key.pem";

        try {
            
        String publicKey = new String(Files.readAllBytes(Paths.get(publicKeyPath)));
        String privateKey = new String(Files.readAllBytes(Paths.get(privateKeyPath)));
        String message = "Hello, world!";
        System.err.println("Message: " + message);

        String encrypted = encryptor.encrypt(message, publicKey);
        System.out.println("Encrypted: " + encrypted);

        String decrypted = encryptor.decrypt(encrypted, privateKey);
        System.out.println("Decrypted: " + decrypted);
        } catch (IOException e) {
            System.err.println("Error: " + e.toString());
        }
    }
}

