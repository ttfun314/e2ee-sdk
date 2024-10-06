class Encryptor {
    static {
        System.loadLibrary("e2ee_sdk");
    }

    public native String encrypt(String msg, String publicKey);

    public native String decrypt(String cipher, String privateKey);
    public native String[] generateKeys(int keySize);    
    public static void main(String[] args) {
        Encryptor encryptor = new Encryptor();

        String[] keyPair = encryptor.generateKeys(2048);
        String privateKey = keyPair[0];
        String publicKey = keyPair[1];

            
        String message = "Hello, world!";
        System.err.println("Message: " + message);

        String encrypted = encryptor.encrypt(message, publicKey);
        System.out.println("Encrypted: " + encrypted);

        String decrypted = encryptor.decrypt(encrypted, privateKey);
        System.out.println("Decrypted: " + decrypted);
        
    }
}

