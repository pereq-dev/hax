use hax_lib as hax;
use libcrux::aead::{self, Algorithm};

const AEAD_KEY_NONCE: usize = Algorithm::key_size(Algorithm::Chacha20Poly1305)
    + Algorithm::nonce_size(Algorithm::Chacha20Poly1305);

const AEAD_KEY_LENGTH: usize = Algorithm::key_size(Algorithm::Chacha20Poly1305);

const EMPTY_AAD: &[u8; 0] = b"";
const RESPONSE_KEY_CONTEXT: &[u8; 12] = b"response-key";

/* Type definitions */
#[derive(Debug)]
pub enum Error {
    CryptoError,
    OtherError,
}

impl From<libcrux::aead::Error> for Error {
    fn from(_value: libcrux::aead::Error) -> Error {
        Error::CryptoError
    }
}

impl From<libcrux::hkdf::Error> for Error {
    fn from(_value: libcrux::hkdf::Error) -> Error {
        Error::CryptoError
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(_value: std::array::TryFromSliceError) -> Error {
        Error::OtherError
    }
}

#[hax::opaque]
pub struct Message(aead::Tag, Vec<u8>);

#[hax::opaque]
pub struct KeyIv(libcrux::aead::Key, libcrux::aead::Iv);

/* Wire formats */
// Serialization is an injective encoding, so the model makes it a constructor,
// and deserialization its inverse: `deserialize_key_iv(serialize_key_iv(k))` is
// `k`, and deserializing anything else fails.
#[hax::pv_constructor]
fn serialize_key_iv(key_iv: &KeyIv) -> Vec<u8> {
    let mut result = Vec::new();
    result.extend_from_slice(key_iv.1 .0.as_ref());
    match &key_iv.0 {
        aead::Key::Chacha20Poly1305(k) => result.extend_from_slice(k.0.as_ref()),
        _ => unimplemented!(),
    }
    result
}

#[hax::pv_inverse_of(serialize_key_iv)]
fn deserialize_key_iv(bytes: &[u8]) -> Result<KeyIv, Error> {
    let iv = aead::Iv::new(&bytes[..12])?;
    let key = aead::Key::from_slice(Algorithm::Chacha20Poly1305, &bytes[12..])?;
    Ok(KeyIv(key, iv))
}

/* Cryptographic functions — redirected to `cryptolib.pvl` */
#[hax::proverif::replace_body("crypto__kdf(ikm, info)")]
fn derive_key_iv(ikm: &[u8], info: &[u8]) -> Result<KeyIv, Error> {
    let key_iv_bytes =
        libcrux::hkdf::expand(libcrux::hkdf::Algorithm::Sha256, ikm, info, AEAD_KEY_NONCE)?;

    let (key_bytes, iv_bytes) = key_iv_bytes.split_at(AEAD_KEY_LENGTH);
    let key =
        libcrux::aead::Key::from_slice(libcrux::aead::Algorithm::Chacha20Poly1305, key_bytes)?;

    let iv = libcrux::aead::Iv(iv_bytes.try_into()?);
    Ok(KeyIv(key, iv))
}

#[hax::proverif::replace_body("crypto__aead_enc(key_iv, message, empty)")]
pub fn encrypt(key_iv: &KeyIv, message: &[u8]) -> Result<Message, Error> {
    let (tag, ctxt) =
        libcrux::aead::encrypt_detached(&key_iv.0, message, aead::Iv(key_iv.1 .0), EMPTY_AAD)?;
    Ok(Message(tag, ctxt))
}

#[hax::proverif::replace_body("crypto__aead_dec(key_iv, message, empty)")]
fn decrypt(key_iv: &KeyIv, message: Message) -> Result<Vec<u8>, Error> {
    libcrux::aead::decrypt_detached(
        &key_iv.0,
        message.1,
        aead::Iv(key_iv.1 .0),
        EMPTY_AAD,
        &message.0,
    )
    .map_err(|_| Error::CryptoError)
}

/* Protocol */
pub fn initiate(ikm: &[u8], psk: &KeyIv) -> Result<(Message, KeyIv), Error> {
    let response_key_iv = derive_key_iv(ikm, RESPONSE_KEY_CONTEXT)?;

    let serialized_responder_key = serialize_key_iv(&response_key_iv);

    let initiator_message = encrypt(psk, &serialized_responder_key)?;

    Ok((initiator_message, response_key_iv))
}

pub fn respond(psk: &KeyIv, payload: &[u8], message: Message) -> Result<Message, Error> {
    let response_key_bytes = decrypt(psk, message)?;

    let response_key_iv = deserialize_key_iv(&response_key_bytes)?;

    let responder_message = encrypt(&response_key_iv, payload)?;

    Ok(responder_message)
}

pub fn finish(message: Message, response_key_iv: &KeyIv) -> Result<Vec<u8>, Error> {
    let response_bytes = decrypt(response_key_iv, message)?;

    Ok(response_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use rand::{rngs::OsRng, RngCore};

        fn random_array<const L: usize>() -> [u8; L] {
            let mut rng = OsRng;
            let mut seed = [0; L];
            rng.try_fill_bytes(&mut seed).unwrap();
            seed
        }
        let payload = b"SECRET";
        let ikm_psk = random_array::<32>();
        let ikm_responder_key = random_array::<32>();

        let psk = derive_key_iv(&ikm_psk, b"pre-shared-key")
            .map_err(|_| Error::CryptoError)
            .unwrap();

        let (initiator_message, response_key) = initiate(&ikm_responder_key, &psk).unwrap();
        let responder_message = respond(&psk, payload, initiator_message).unwrap();
        let initiator_finish = finish(responder_message, &response_key).unwrap();
        assert_eq!(payload.to_vec(), initiator_finish);
    }
}
