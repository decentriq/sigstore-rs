Continuous integration | Docs | License | Crate version | Crate downloads
 ----------------------|------|---------|---------------|-----------------
 [![Continuous integration](https://github.com/sigstore/sigstore-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/sigstore/sigstore-rs/actions/workflows/tests.yml) | [![Docs](https://img.shields.io/badge/docs-%20-blue)](https://docs.rs/sigstore/latest/sigstore) |  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache2.0-brightgreen.svg)](https://opensource.org/licenses/Apache-2.0) | [![Crate version](https://img.shields.io/crates/v/sigstore?style=flat-square)](https://crates.io/crates/sigstore) | [![Crate downloads](https://img.shields.io/crates/d/sigstore?style=flat-square)](https://crates.io/crates/sigstore)


A crate to interact with [sigstore](https://sigstore.dev/).

This crate is under active development and will not be considered
stable until the 1.0 release.

## Features

### Cosign Sign and Verify

The crate implements the following verification mechanisms:

  * Sign using a cosign key and store the signature in a registry
  * Verify using a given key
  * Verify bundle produced by transparency log (Rekor)
  * Verify signature produced in keyless mode, using Fulcio Web-PKI

Signature annotations and certificate email can be provided at verification time.

### Fulcio Integration

For use with Fulcio ephemeral key signing, an OpenID connect API is available,
along with a fulcio client implementation.

### Rekor Client

All rekor client APIs can be leveraged to interact with the transparency log.

### Key Interface

Cryptographic key management with the following key interfaces:

* Generate a key pair
* Sign data
* Verify signature
* Export public / (encrypted) private key in PEM / DER format
* Import public / (encrypted) private key in PEM / DER format

#### Known limitations

* The crate does not handle verification of attestations yet.

## Examples

The `examples` directory contains demo programs using the library.

  * [`openidflow`](examples/openidflow/README.md)
  * [`key_interface`](examples/key_interface/README.md)
  * [`rekor`](examples/rekor/README.md)
  * [`cosign/verify`](examples/cosign/verify/README.md)
  * [`cosign/verify-blob`](examples/cosign/verify-blob/README.md)
  * [`cosign/verify-bundle`](examples/cosign/verify-bundle/README.md)
  * [`cosign/sign`](examples/cosign/sign/README.md)

Each example can be executed with the `cargo run --example <name>` command.

For example, `openidconnect` can be run with the following command:

```bash
cargo run --example openidconnect
```

## Contributing

Contributions are welcome! Please see the [contributing guidelines](CONTRIBUTORS.md)
for more information.

## Security

Should you discover any security issues, please refer to sigstores [security
process](https://github.com/sigstore/community/security/policy)

## Network-free Rekor models (Decentriq fork)

This branch is based on the stable upstream **v0.14.0** release. Use
`default-features = false, features = ["rekor-models"]` for Rekor data models,
checkpoint parsing and Merkle-proof verification without HTTP/browser clients
or AWS-LC. The public proof methods require a `crypto::SignatureVerifier` supplied
by the caller; they do not skip signature verification. The caller must obtain
its verification key through a trusted channel and enforce the expected algorithm.

The normal `rekor` feature still enables the HTTP APIs and upstream crypto
implementation. `rekor-apis` enables just the HTTP/model layer. `crypto` retains
the upstream AWS-LC implementation and is enabled by default and by features
that require it (including `cert` and `fulcio`). Disabling default features is
therefore essential for the network-free configuration.

The former fork's network split and public Merkle helpers are carried forward;
its optional browser dependency, Merkle algorithms and 64-bit tree sizes are
already present upstream. Public checkpoint helpers now use upstream's names:
`SignedCheckpoint` is the signed envelope and `Checkpoint` is its note.

Validation of this configuration:

```sh
cargo test --lib --test rekor_models --no-default-features --features rekor-models
cargo test --lib --no-default-features --features rekor-models,crypto
cargo check --lib --no-default-features --features rekor-models --target x86_64-fortanix-unknown-sgx
cargo check --lib --no-default-features --features rekor-models --target wasm32-unknown-unknown
```

This is a portability/API split, not a claim that every optional upstream
feature is free of advisories. In particular, enabling `sign` still selects the
RustCrypto dependencies of the upstream v0.14.0 release. They are not in the
model-only graph.
