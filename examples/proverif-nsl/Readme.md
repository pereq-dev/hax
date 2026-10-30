# The Needham–Schroeder–Lowe protocol in ProVerif

This crate implements the Needham–Schroeder–Lowe (NSL) public-key protocol,
and extracts it to ProVerif with hax to check its security.

```
A -> B: {na, pk_A}pk_B
B -> A: {na, nb, pk_B}pk_A
A -> B: {nb}pk_B
```

Each party is identified by its public key, and both derive a session key from
the two nonces. The responder's identity in the second message is Lowe's fix:
without it, the original Needham–Schroeder protocol has a man-in-the-middle
attack.

The protocol is generic over a `PublicKeyEncryption` trait, implemented with
HPKE (X25519, HKDF-SHA256, ChaCha20Poly1305). The crate does not implement
message transport.

## Modelling the protocol

ProVerif works on symbolic terms, so a few functions get an abstract model
instead of their Rust code:

- **The public-key encryption** is the `PublicKeyEncryption` trait. The
  `keep-trait-calls` key of `hax.toml` keeps the calls of its items as they
  are, rather than resolving them to the HPKE impl, and `nsl_crypto.pvl`
  models them once for every implementation. The HPKE impls are
  `#[hax_lib::opaque]`: their code is not extracted.
- **The session key** is `#[hax_lib::pv_extern]`: a one-way function of the
  two nonces, computable from them and revealing nothing about them.
- **The message encoding** is `#[hax_lib::pv_constructor]`, an injective
  encoding, and the decoding is `#[hax_lib::pv_inverse_of(encode)]`, which
  recovers exactly what was encoded and fails on anything else.

The `proverif-psk` example shows the other way of modelling cryptography: a
`#[hax_lib::proverif::replace_body(...)]` per function, calling the ideal
functionality of hax's `cryptolib.pvl`.

## Extracting and analysing

To extract the model and run the analysis, from the crate root:

```
make check
```

`cargo hax extract nsl` writes `./proofs/nsl/proverif/extraction/lib.pvl`,
and `proofs/nsl/proverif/analysis.pv` runs an honest initiator and an honest
responder in any number of sessions, against an attacker that knows their
public keys, chooses whom the initiator talks to, and has keys of its own. It
checks that

- both roles can complete the protocol,
- each party authenticates the other, session by session,
- the session key of a session between the honest parties stays secret.

`make check` fails unless the verdicts are the expected ones:

```
RESULT not event(InitiatorDone(a,b_3,k)) is false.
RESULT not event(ResponderDone(a,b_3,k)) is false.
RESULT inj-event(InitiatorDone(a,b_3,k)) ==> inj-event(ResponderCommits(a,b_3,k)) is true.
RESULT inj-event(ResponderDone(a,b_3,k)) ==> inj-event(InitiatorCommits(a,b_3,k)) is true.
RESULT not attacker(INITIATOR_SECRET[]) is true.
RESULT not attacker(RESPONDER_SECRET[]) is true.
```

Dropping the initiator's check of the responder's identity in
`finish_initiator` brings Lowe's attack back: the responder's authentication
of the initiator and the secrecy of its session key then fail.
