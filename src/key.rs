extern crate openssl;

use openssl::sign::Signer;
use openssl::hash::MessageDigest;
use openssl::bn::BigNumContext;
use openssl::ec::*;
use openssl::pkey::PKey;
use openssl::nid;

pub struct Key {
    pkey: PKey,
}

impl Key {
    pub fn new() -> Result<Key, openssl::error::ErrorStack> {
        let group = EcGroup::from_curve_name(nid::SECP256K1).unwrap();
        let key = EcKey::generate(&group)?;
        Ok(Key { pkey: PKey::from_ec_key(key)? })
    }

    pub fn public_key(self) -> Result<Option<Vec<u8>>, openssl::error::ErrorStack> {
        let group = EcGroup::from_curve_name(nid::SECP256K1)?;
        let form = openssl::ec::POINT_CONVERSION_UNCOMPRESSED;
        let mut ctx = BigNumContext::new()?;
        Ok(match self.pkey.ec_key()?.public_key() {
            Some(k) => Some(k.to_bytes(&group, form, &mut ctx)?),
            None => None,
        })
    }

    pub fn private_key(self) -> Result<Option<Vec<u8>>, openssl::error::ErrorStack> {
        Ok(match self.pkey.ec_key()?.private_key() {
            Some(k) => Some(k.to_vec()),
            None => None,
        })
    }

    pub fn sign(self, msg: &str) -> Result<Vec<u8>, openssl::error::ErrorStack> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.pkey)?;
        signer.update(msg.as_bytes())?;
        signer.finish()
    }
}
