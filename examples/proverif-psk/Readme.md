# A hax ProVerif example

This crate demonstrates an example of ProVerif extraction using hax.

The crate provides functions for implementing a simplistic pre-shared-key (PSK) based protocol
between an initiator and receiver, which is defined as follows:
```
Initiator(psk: AEADKey): 
    let response_key = AEAD.KeyGen()
    let message = AEAD.Encrypt(psk, response_key)

Initiator -> Responder: message

Responder(psk: AEADKey, payload: &[u8]):
    let response_key = AEAD.Decrypt(psk, message)
    let response = AEAD.Encrypt(response_key, payload)
           
Responder -> Initiator: response

Initiator(response_key, response): 
    let output = AEAD.Decrypt(response_key, response)
    return output
```

The crate does not implement message transport, only the initiator and
responder protocol logic.

A handwritten ProVerif model of this protocol is included in `psk.pv` for comparison.

### Modelling the cryptography and the wire format
ProVerif works on symbolic terms, so some functions get an abstract model
instead of their Rust body:

- The cryptographic operations (key derivation, encryption, decryption) are
  `#[hax_lib::proverif::replace_body(...)]`: their body becomes a call to the
  ideal functionality of `cryptolib.pvl`. hax's other backends can prove that
  the implementations are correct.
- The wire format is `#[hax_lib::pv_constructor]` for serialization, an
  injective encoding, and `#[hax_lib::pv_inverse_of(serialize_key_iv)]` for
  deserialization, which then recovers exactly what was serialized and fails
  on anything else.

The `proverif-nsl` example models its cryptography through a trait instead,
and uses `#[hax_lib::pv_extern]` for its key derivation.

## Extracting into ProVerif
To obtain a ProVerif model of the protocol logic functions, run the `psk`
proof scenario declared in `hax.toml`:
```
cargo hax extract psk
```
This will generate a file `./proofs/psk/proverif/extraction/lib.pvl`.

## Running a Basic Analysis on the Model
We have provided a handwritten file
`./proofs/psk/proverif/analysis.pv`, which models the protocol
using the extracted functions in `lib.pvl` and uses ProVerif to verify

- that initiator and receiver can both complete the protocol, as well as
- confidentiality of the pre-shared key and the protocol payload

To let ProVerif perform the analysis, from the crate root, run `make check`.
It fails unless the verdicts are the expected ones below, and runs:

```
proverif -lib ../../hax-lib/proof-libs/proverif/primitives \
         -lib ../../hax-lib/proof-libs/proverif/result \
         -lib ../../hax-lib/proof-libs/proverif/cryptolib \
         -lib ./proofs/psk/proverif/extraction/missingdecl \
         -lib ./proofs/psk/proverif/extraction/lib \
         ./proofs/psk/proverif/analysis.pv
```

The expected final output is
```
--------------------------------------------------------------
Verification summary:

Query not event(InitiatorFinished(initiator_result)) is false.

Query not event(ResponderFinished(responder_result)) is false.

Query not attacker(PSK[]) is true.

Query not attacker(SECRET_PAYLOAD[]) is true.

--------------------------------------------------------------
```

