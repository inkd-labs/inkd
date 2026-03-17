# Engineering note 142

Context: fix(core): reject a revoke on an already revoked attestation earlier.

- scope: inkd
- status: merged

This note records a small, self-contained change so that future
maintainers can track why a knob was touched without needing to walk
the full commit graph. It does not introduce new runtime behaviour.
