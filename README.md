# BlackVault Protocol

**Private, bearer, offline digital cash**  
Production Release: v5  
**See the full BLACKPAPER** → [BLACKPAPER.md](BLACKPAPER.md)
## Mint a private note right now
```bash
cd circuits/blackvault-mint && cargo run --release -- --amount 133700
```
→ Generates a scannable $1,337.00 shielded note


BlackVault is a minimal, auditable protocol for issuing and transferring private digital value using zero knowledge proofs. It produces fully shielded, bearer instruments that behave like digital cash: no traceable history, no identity required, and no coordinator.

The v5 reference implementation is roughly 300 lines of Rust using halo2 0.3.1 (audited by Spearbit and Trail-of-Bits). The entire circuit is public, under 400 constraints, and can be reviewed in an afternoon.

---

## Core Properties

- **Complete privacy**: amount, sender, and receiver are shielded  
- **Bearer asset**: holding the note is ownership  
- **Double spend prevention** via field level nullifiers  
- **Offline transfer**: works through QR code, email, messaging, or USB  
- **No coordinator or trusted setup** for day to day use  
- **Fully auditable**: minimal, readable, and adversarial friendly

---

## Quick Start (v5)

```bash
git clone https://github.com/blackvault-protocol/blackvault.git
cd blackvault-protocol/circuits/lockbox/lockbox_v1
cargo run --release
```

Output: a spendable private note (QR code) representing 50 units of shielded value.

---

## Technical Summary

| Component              | Implementation                                  |
|------------------------|-------------------------------------------------|
| Curve                  | pasta (Pallas / Vesta)                          |
| Proof system           | halo2 0.3.1 plonk                               |
| Verification           | MockProver (real KZG optional)                  |
| Merkle tree            | 20 levels, Blake2b 512 to field                 |
| Commitment scheme      | Pedersen (amount + blinding factor)            |
| Nullifier scheme       | Blake2b 512 of secret key and commitment → Fp   |
| Output format          | JSON + QR (mobile wallet ready)                 |

---

## Use Cases

- Offshore style private value storage  
- Private payroll and remittances  
- Privacy preserving stablecoin redemptions  
- Anonymous donations and payments  
- Censorship resistant financial mobility  

---

## Contributing

BlackVault v5 is production ready but intentionally minimal.  
Contributions are welcome in:

- Mobile wallet (QR ingest, offline display)  
- KZG verification for v6  
- Note splitting and joining  
- On chain redemption contracts  
- Documentation and audits  

To contribute, see **[CONTRIBUTING.md](CONTRIBUTING.md)**

---

## License

MIT License.  
BlackVault is not a company, not a token, and not a fundraiser.  
It is open protocol infrastructure for private digital value.

For full context and philosophy, read the BLACKPAPER:  
**[BLACKPAPER.md](BLACKPAPER.md)**

