package com.x.e2ee;
import android.content.Context;
import android.content.SharedPreferences;
import androidx.security.crypto.EncryptedSharedPreferences;
import androidx.security.crypto.MasterKeys;

import java.io.IOException;
import java.security.GeneralSecurityException;

public class CryptoSDK {

    private final SharedPreferences sharedPreferences;
    private final Encryptor encryptor;

    public CryptoSDK(Context context) throws GeneralSecurityException, IOException {
        String masterKeyAlias = MasterKeys.getOrCreate(MasterKeys.AES256_GCM_SPEC);
        sharedPreferences = EncryptedSharedPreferences.create(
                "secret_shared_prefs",  // File name for preferences
                masterKeyAlias,         // The master key alias
                context,                // Context
                EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,  // Key encryption
                EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM // Value encryption
        );
        encryptor = new Encryptor();
    }

    // Generate key pair if not exists, save keys to encrypted shared preferences where they are not accessible to other apps or users
    public void init() {
        if (getPublicKey() != null) {
            return;
        }
        SharedPreferences.Editor editor = sharedPreferences.edit();
        String[] keyPair = encryptor.generateKeys(2048);
        String privateKey = keyPair[0];
        String publicKey = keyPair[1];

        editor.putString("private_key", privateKey);
        editor.putString("public_key", publicKey);
        editor.apply();  // Save asynchronously
    }

    private String getPrivateKey() {
        return sharedPreferences.getString("private_key", null);
    }

    public String getPublicKey() {
        return sharedPreferences.getString("public_key", null);
    }

    // Encrypt message with the provided public key
    public String encrypt(String msg, String publicKey) {
        return encryptor.encrypt(msg, publicKey);
    }

    // Decrypt message with saved private key
    public String decrypt(String cipher) {
        String privateKey = getPrivateKey();
        return encryptor.decrypt(cipher, privateKey);
    }
}
