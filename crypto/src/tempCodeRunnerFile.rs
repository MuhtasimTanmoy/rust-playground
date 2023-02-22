   // use crypto::ed25519;
    // use rand::RngCore;
    // use rand::rngs::OsRng;
    // use super::*;
    // #[test]
    // fn test_signature() {
    //     let mut key: [u8; 32] = [0; 32];
    //     OsRng.fill_bytes(&mut key);
    //     let (secrect_key, public_key) = ed25519::keypair(&key);
    //     let secret_key = secrect_key.to_vec();
    //     let public_key = public_key.to_vec();

    //     let signature =  ed25519::signature("test".as_bytes(), &secret_key);
    //     assert!(ed25519::verify(
    //         "test".as_bytes(),
    //         &public_key,
    //         &signature
    //     ));
    // }
