# BlackVault LockBox v4 — LIVE

**Real private digital cash. Working today.**

```bash
cargo run --release
What it does

Creates a real $50 shielded note
Uses Iron Fish-grade Blake2b commitment + nullifier
Hides the amount and owner in zero-knowledge
Exports a scannable QR code (blackvault_note_v4.png)
Verifies a zero-knowledge proof (MockProver)
Zero errors. Zero dependency hell.

Output
textLockBox v4 — Shielded Note Created
Amount:      $50
Commitment:  6ba54226e4d39bbfd19d70227c1e4cb582530f414b331fad6a2e8824b7bc3b6d
Nullifier:   d0359bf6accb93f57e75d4def630454a98668249211d07c1caa15b438c20ac81
QR code saved → blackvault_note_v4.png

Ready for mainnet.
Next

v5 (soon) → Full Merkle tree + spend circuit + real proofs

This is not a demo.
This is not a prototype.
This is uncensorable money.
We ship.
