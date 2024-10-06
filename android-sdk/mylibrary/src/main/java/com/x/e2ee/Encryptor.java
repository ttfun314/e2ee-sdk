package com.x.e2ee;

class Encryptor {
    static {
        System.loadLibrary("e2ee_sdk");
    }

    public native String encrypt(String msg, String publicKey);

    public native String decrypt(String cipher, String privateKey);

    public native String[] generateKeys(int keySize);

}

