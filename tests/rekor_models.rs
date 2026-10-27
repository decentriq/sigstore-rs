#![cfg(feature = "rekor-models")]

use std::cell::Cell;

use sigstore::crypto::merkle::{
    MerkleProofVerifier, Rfc6269Default, Rfc6269HasherTrait, hex_to_hash_output,
};
use sigstore::crypto::{Signature, SignatureVerifier};
use sigstore::errors::{Result, SigstoreError};
use sigstore::rekor::models::InclusionProof;
use sigstore::rekor::models::checkpoint::{Checkpoint, CheckpointSignature, SignedCheckpoint};

#[test]
fn public_merkle_helpers_are_available_without_a_crypto_backend() {
    let leaf = Rfc6269Default::hash_leaf(b"entry");
    let root = Rfc6269Default::root_from_inclusion_proof(0, &leaf, 1, &[]).unwrap();
    assert_eq!(*root, leaf);
    assert_eq!(hex_to_hash_output(hex::encode(&*root)).unwrap(), leaf);
}

#[test]
fn checkpoint_encoding_preserves_64_bit_tree_sizes() {
    let note = Checkpoint {
        origin: "test.example".into(),
        size: 1u64 << 40,
        hash: [1; 32],
        other_content: vec![],
    };
    assert_eq!(Checkpoint::unmarshal(&note.marshal()).unwrap(), note);
}

#[test]
fn inclusion_proofs_cannot_skip_the_callers_signature_verifier() {
    struct RejectAll(Cell<usize>);
    impl SignatureVerifier for RejectAll {
        fn verify_signature(&self, _: Signature<'_>, _: &[u8]) -> Result<()> {
            self.0.set(self.0.get() + 1);
            Err(SigstoreError::PublicKeyVerificationError)
        }
    }

    let entry = b"entry";
    let root: [u8; 32] = Rfc6269Default::hash_leaf(entry).into();
    let checkpoint = SignedCheckpoint {
        note: Checkpoint {
            origin: "test.example".into(),
            size: 1,
            hash: root,
            other_content: vec![],
        },
        signatures: vec![CheckpointSignature {
            name: "test.example".into(),
            key_fingerprint: [0; 4],
            raw: vec![1, 2, 3],
        }],
    };
    let mut proof = InclusionProof::new(0, root, 1, vec![], Some(checkpoint));
    let verifier = RejectAll(Cell::new(0));
    assert!(matches!(
        proof.verify(entry, &verifier),
        Err(SigstoreError::CheckpointSignatureVerificationError)
    ));
    assert_eq!(verifier.0.get(), 1);

    // An empty signature list must not turn proof verification into authentication.
    proof.checkpoint.as_mut().unwrap().signatures.clear();
    assert!(proof.verify(entry, &verifier).is_err());
    assert_eq!(verifier.0.get(), 1);
}
