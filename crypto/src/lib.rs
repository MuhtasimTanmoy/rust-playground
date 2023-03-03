use ed25519::signature::{Signer, Verifier};

pub struct HelloSigner<S>
where
    S: Signer<ed25519::Signature> {
    pub signing_key: S
}

impl<S> HelloSigner<S>
where
    S: Signer<ed25519::Signature> {
    pub fn sign(&self, person: &str) -> ed25519::Signature {
        // NOTE: use `try_sign` if you'd like to be able to handle
        // errors from external signing services/devices (e.g. HSM/KMS)
        // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
        self.signing_key.sign(format_message(person).as_bytes())
    }
}

pub struct HelloVerifier<V> {
    pub verifying_key: V
}

impl<V> HelloVerifier<V>
where
    V: Verifier<ed25519::Signature> {
    pub fn verify(
        &self,
        person: &str,
        signature: &ed25519::Signature
    ) -> Result<(), ed25519::Error> {
        self.verifying_key.verify(format_message(person).as_bytes(), signature)
    }
}

fn format_message(person: &str) -> String {
    format!("Hello, {}!", person)
}


#[cfg(test)]
mod test {
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

    
    use super::*;
    #[test]
    fn updated_test() {
        use ring_compat::signature::{
            ed25519::{Signature, SigningKey, VerifyingKey},
            Signer, Verifier
        };
        use rand_core::{OsRng, RngCore}; // Requires the `std` feature of `rand_core`
        
        /// `HelloSigner` defined above instantiated with *ring* as
        /// the signing provider.
        pub type RingHelloSigner = HelloSigner<SigningKey>;
        
        let mut ed25519_seed = [0u8; 32];
        OsRng.fill_bytes(&mut ed25519_seed);
        
        let signing_key = SigningKey::from_seed(&ed25519_seed).unwrap();
        let verifying_key = signing_key.verifying_key();
        
        let signer = RingHelloSigner { signing_key };
        let person = "Joe"; // Message to sign
        let signature = signer.sign(person);
        
        /// `HelloVerifier` defined above instantiated with *ring*
        /// as the signature verification provider.
        pub type RingHelloVerifier = HelloVerifier<VerifyingKey>;
        
        let verifier = RingHelloVerifier { verifying_key };
        assert!(verifier.verify(person, &signature).is_ok());
    }
}