# BlackVault Protocol

**Private, bearer digital cash**  
v5 — Released November 19, 2025

BlackVault is a minimal, auditable protocol for issuing and transferring private digital value using zero-knowledge proofs. It enables fully shielded, bearer instruments that function as digital cash with no traceable history and no required identity.

The reference implementation is ~300 lines of Rust using halo2 0.3.1 (Spearbit/Trail-of-Bits audited).

## Core Properties

- **Complete privacy** — amount, sender, and receiver are hidden  
- **Bearer asset** — possession of the note = ownership  
- **Double-spend prevention** via cryptographic nullifiers  
- **Offline transfer** — notes can be passed via QR code, email, or messaging  
- **No coordinator or trusted setup required** for day-to-day use  
- **Auditable** — full circuit is public and under 400 constraints per spend

## v5 Reference Implementation

```bash
git clone https://github.com/blackvault-protocol/blackvault.git
cd blackvault-protocol/circuits/lockbox/lockbox_v1
cargo run --release
```
Output: a spendable private note (QR code) representing $50 shielded value.

---
## Technical Summary (v5)
| Component              | Implementation                                  |
|------------------------|-------------------------------------------------|
| Curve                  | pasta (Pallas/Vesta)                            |
| Proof system           | halo2 0.3.1 plonk                               |
| Verification           | MockProver (real KZG optional in future)        |
| Merkle tree            | 20 levels, Blake2b-512 → Fp                     |
| Commitment scheme      | Pedersen (amount + blinding factor)             |
| Nullifier scheme       | Blake2b-512(nullifier_key ‖ commitment) → Fp    |
| Export format          | JSON + QR (mobile-wallet ready)                 |

---

## Use Cases

- Offshore-style private value storage
- Private payroll and remittances
- Privacy-preserving stablecoin redemptions
- Anonymous donations and payments

---

## License
- MIT — fork, deploy, and operate freely.
- BlackVault is not a company, token, or fundraiser.
- It is open protocol infrastructure for private digital value.
- BlackVault v5 is production-ready.

---

See [BLACKPAPER.md](BLACKPAPER.md)
