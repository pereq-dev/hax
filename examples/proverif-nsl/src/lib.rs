//! The Needham–Schroeder–Lowe public-key protocol:
//!
//! ```text
//! A -> B: {na, pk_A}pk_B
//! B -> A: {na, nb, pk_B}pk_A
//! A -> B: {nb}pk_B
//! ```
//!
//! Each party is identified by its public key. The responder's identity in the
//! second message is Lowe's fix to the original Needham–Schroeder protocol.
//! Both parties then derive a session key from the two nonces.
//!
//! The protocol is generic over the public-key encryption scheme; [`Hpke`]
//! implements it with HPKE.
use hax_lib as hax;
use libcrux::hpke::{self, aead::AEAD, kdf::KDF, kem::KEM, HPKECiphertext, HPKEConfig, Mode};

const KEY_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 32;
const SESSION_KEY_INFO: &[u8; 15] = b"nsl session key";

#[derive(Debug)]
pub enum Error {
    Crypto,
    Decoding,
    UnexpectedMessage,
}

/* Keys and nonces */
#[hax::opaque]
#[derive(Clone, PartialEq, Debug)]
pub struct PublicKey(Vec<u8>);

#[hax::opaque]
pub struct SecretKey(Vec<u8>);

#[hax::opaque]
pub struct Ciphertext(Vec<u8>);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Nonce(pub [u8; NONCE_LENGTH]);

/* Public-key encryption */
/// A public-key encryption scheme. `nsl_crypto.pvl` models its operations
/// once for every implementation: the extraction keeps the calls of this
/// trait's items as they are (`keep-trait-calls` in `hax.toml`).
pub trait PublicKeyEncryption {
    fn public_key(sk: &SecretKey) -> Result<PublicKey, Error>;
    fn encrypt(
        pk: &PublicKey,
        plaintext: &[u8],
        randomness: [u8; KEY_LENGTH],
    ) -> Result<Ciphertext, Error>;
    fn decrypt(sk: &SecretKey, ciphertext: &Ciphertext) -> Result<Vec<u8>, Error>;
}

/// HPKE in base mode, with X25519, HKDF-SHA256 and ChaCha20Poly1305. Its
/// code is not extracted: the model of the trait stands for it.
pub struct Hpke;

#[hax::opaque]
impl Hpke {
    const ENCAPSULATION_LENGTH: usize = 32;
    const INFO: &[u8; 3] = b"nsl";

    fn config() -> HPKEConfig {
        HPKEConfig(
            Mode::mode_base,
            KEM::DHKEM_X25519_HKDF_SHA256,
            KDF::HKDF_SHA256,
            AEAD::ChaCha20Poly1305,
        )
    }

    /// The key pair that `randomness` determines.
    pub fn generate_key_pair(
        randomness: [u8; KEY_LENGTH],
    ) -> Result<(SecretKey, PublicKey), Error> {
        let (sk, pk) =
            hpke::kem::GenerateKeyPair(KEM::DHKEM_X25519_HKDF_SHA256, randomness.to_vec())
                .map_err(|_| Error::Crypto)?;
        Ok((SecretKey(sk), PublicKey(pk)))
    }
}

#[hax::opaque]
impl PublicKeyEncryption for Hpke {
    fn public_key(sk: &SecretKey) -> Result<PublicKey, Error> {
        libcrux::ecdh::secret_to_public(libcrux::ecdh::Algorithm::X25519, &sk.0)
            .map(PublicKey)
            .map_err(|_| Error::Crypto)
    }

    fn encrypt(
        pk: &PublicKey,
        plaintext: &[u8],
        randomness: [u8; KEY_LENGTH],
    ) -> Result<Ciphertext, Error> {
        let HPKECiphertext(mut encapsulation, ciphertext) = hpke::HpkeSeal(
            Self::config(),
            &pk.0,
            Self::INFO,
            b"",
            plaintext,
            None,
            None,
            None,
            randomness.to_vec(),
        )
        .map_err(|_| Error::Crypto)?;
        encapsulation.extend_from_slice(&ciphertext);
        Ok(Ciphertext(encapsulation))
    }

    fn decrypt(sk: &SecretKey, ciphertext: &Ciphertext) -> Result<Vec<u8>, Error> {
        if ciphertext.0.len() < Self::ENCAPSULATION_LENGTH {
            return Err(Error::Crypto);
        }
        let (encapsulation, ciphertext) = ciphertext.0.split_at(Self::ENCAPSULATION_LENGTH);
        let ciphertext = HPKECiphertext(encapsulation.to_vec(), ciphertext.to_vec());
        hpke::HpkeOpen(Self::config(), &ciphertext, &sk.0, Self::INFO, b"", None, None, None)
            .map_err(|_| Error::Crypto)
    }
}

/* Session key */
/// The session key both parties derive from the two nonces. The model makes
/// it a one-way function of them: computable from the nonces, and revealing
/// nothing about them.
#[hax::pv_extern]
pub fn session_key(initiator_nonce: &Nonce, responder_nonce: &Nonce) -> Result<[u8; 32], Error> {
    let mut ikm = initiator_nonce.0.to_vec();
    ikm.extend_from_slice(&responder_nonce.0);
    let key = libcrux::hkdf::hkdf(
        libcrux::hkdf::Algorithm::Sha256,
        b"",
        &ikm,
        SESSION_KEY_INFO,
        32,
    )
    .map_err(|_| Error::Crypto)?;
    key.try_into().map_err(|_| Error::Crypto)
}

/* Messages */
pub enum Message {
    One {
        nonce: Nonce,
        initiator: PublicKey,
    },
    Two {
        initiator_nonce: Nonce,
        responder_nonce: Nonce,
        responder: PublicKey,
    },
    Three {
        nonce: Nonce,
    },
}

// The encoding is injective, so the model makes it a constructor, and decoding
// its inverse: decoding gives back exactly what was encoded, and fails on
// anything else.
#[hax::pv_constructor]
fn encode(message: &Message) -> Vec<u8> {
    let mut bytes = Vec::new();
    match message {
        Message::One { nonce, initiator } => {
            bytes.push(1);
            bytes.extend_from_slice(&nonce.0);
            bytes.extend_from_slice(&initiator.0);
        }
        Message::Two {
            initiator_nonce,
            responder_nonce,
            responder,
        } => {
            bytes.push(2);
            bytes.extend_from_slice(&initiator_nonce.0);
            bytes.extend_from_slice(&responder_nonce.0);
            bytes.extend_from_slice(&responder.0);
        }
        Message::Three { nonce } => {
            bytes.push(3);
            bytes.extend_from_slice(&nonce.0);
        }
    }
    bytes
}

#[hax::pv_inverse_of(encode)]
fn decode(bytes: &[u8]) -> Result<Message, Error> {
    let nonce = |at: usize| -> Result<Nonce, Error> {
        let slice = bytes.get(at..at + NONCE_LENGTH).ok_or(Error::Decoding)?;
        Ok(Nonce(slice.try_into().map_err(|_| Error::Decoding)?))
    };
    let key = |at: usize| -> Result<PublicKey, Error> {
        let slice = bytes.get(at..at + KEY_LENGTH).ok_or(Error::Decoding)?;
        Ok(PublicKey(slice.to_vec()))
    };
    match bytes.first() {
        Some(1) if bytes.len() == 1 + NONCE_LENGTH + KEY_LENGTH => Ok(Message::One {
            nonce: nonce(1)?,
            initiator: key(1 + NONCE_LENGTH)?,
        }),
        Some(2) if bytes.len() == 1 + 2 * NONCE_LENGTH + KEY_LENGTH => Ok(Message::Two {
            initiator_nonce: nonce(1)?,
            responder_nonce: nonce(1 + NONCE_LENGTH)?,
            responder: key(1 + 2 * NONCE_LENGTH)?,
        }),
        Some(3) if bytes.len() == 1 + NONCE_LENGTH => Ok(Message::Three { nonce: nonce(1)? }),
        _ => Err(Error::Decoding),
    }
}

/* Protocol */
/// The initiator's state after the first message.
pub struct Initiator {
    sk: SecretKey,
    peer: PublicKey,
    nonce: Nonce,
}

/// The responder's state after the second message.
pub struct Responder {
    pub initiator: PublicKey,
    initiator_nonce: Nonce,
    nonce: Nonce,
}

/// The initiator `sk` starts a session with `peer`: the first message.
pub fn initiate<P: PublicKeyEncryption>(
    sk: SecretKey,
    peer: PublicKey,
    nonce: Nonce,
    randomness: [u8; KEY_LENGTH],
) -> Result<(Initiator, Ciphertext), Error> {
    let initiator = P::public_key(&sk)?;
    let message = encode(&Message::One { nonce, initiator });
    let ciphertext = P::encrypt(&peer, &message, randomness)?;
    Ok((Initiator { sk, peer, nonce }, ciphertext))
}

/// The responder `sk` answers a first message: the second message.
pub fn respond<P: PublicKeyEncryption>(
    sk: &SecretKey,
    nonce: Nonce,
    ciphertext: &Ciphertext,
    randomness: [u8; KEY_LENGTH],
) -> Result<(Responder, Ciphertext), Error> {
    let responder = P::public_key(sk)?;
    match decode(&P::decrypt(sk, ciphertext)?)? {
        Message::One {
            nonce: initiator_nonce,
            initiator,
        } => {
            let message = encode(&Message::Two {
                initiator_nonce,
                responder_nonce: nonce,
                responder,
            });
            let ciphertext = P::encrypt(&initiator, &message, randomness)?;
            let state = Responder {
                initiator,
                initiator_nonce,
                nonce,
            };
            Ok((state, ciphertext))
        }
        _ => Err(Error::UnexpectedMessage),
    }
}

/// The initiator checks the second message, which must carry its nonce and
/// come from its peer: the third message, and the session key.
pub fn finish_initiator<P: PublicKeyEncryption>(
    state: Initiator,
    ciphertext: &Ciphertext,
    randomness: [u8; KEY_LENGTH],
) -> Result<(Ciphertext, [u8; 32]), Error> {
    match decode(&P::decrypt(&state.sk, ciphertext)?)? {
        Message::Two {
            initiator_nonce,
            responder_nonce,
            responder,
        } if initiator_nonce == state.nonce && responder == state.peer => {
            let message = encode(&Message::Three {
                nonce: responder_nonce,
            });
            let ciphertext = P::encrypt(&state.peer, &message, randomness)?;
            Ok((ciphertext, session_key(&initiator_nonce, &responder_nonce)?))
        }
        _ => Err(Error::UnexpectedMessage),
    }
}

/// The responder checks the third message, which must carry its nonce: the
/// session key.
pub fn finish_responder<P: PublicKeyEncryption>(
    sk: &SecretKey,
    state: &Responder,
    ciphertext: &Ciphertext,
) -> Result<[u8; 32], Error> {
    match decode(&P::decrypt(sk, ciphertext)?)? {
        Message::Three { nonce } if nonce == state.nonce => {
            session_key(&state.initiator_nonce, &state.nonce)
        }
        _ => Err(Error::UnexpectedMessage),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::OsRng, RngCore};

    fn random() -> [u8; 32] {
        let mut bytes = [0; 32];
        OsRng.fill_bytes(&mut bytes);
        bytes
    }

    #[test]
    fn a_session_completes() {
        let (sk_a, pk_a) = Hpke::generate_key_pair(random()).unwrap();
        let (sk_b, pk_b) = Hpke::generate_key_pair(random()).unwrap();
        assert_eq!(Hpke::public_key(&sk_a).unwrap(), pk_a);

        let (initiator, message_1) =
            initiate::<Hpke>(sk_a, pk_b, Nonce(random()), random()).unwrap();
        let (responder, message_2) =
            respond::<Hpke>(&sk_b, Nonce(random()), &message_1, random()).unwrap();
        assert_eq!(responder.initiator, pk_a);
        let (message_3, key_a) = finish_initiator::<Hpke>(initiator, &message_2, random()).unwrap();
        let key_b = finish_responder::<Hpke>(&sk_b, &responder, &message_3).unwrap();
        assert_eq!(key_a, key_b);
    }
}
