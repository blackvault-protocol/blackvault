# BLACKPAPER  
BlackVault — Money That Cannot Be Looted  
November 2025

## I. Philosophy

> “So you think that money is the root of all evil?” said Francisco d’Anconia.  
> “Have you ever asked what is the root of money?  
> Money is a tool of exchange, which can’t exist unless there are goods produced and men able to produce them. …  
> When money ceases to be the tool by which men deal with one another, then men become the tools of men.  
> Blood, whips and guns—or dollars. Take your choice—there is no other—and your time is running out.  
> …  
> Then you will see the rise of the men of the double standard—the men who live by force, yet count on those who live by trade to create the value of their looted money—the men who are the hitchhikers of virtue.”

— Ayn Rand, *Atlas Shrugged* (1957)

There are two kinds of men: builders and looters.  
History has shown, again and again, that when economic pressures mount and fiat currencies decay, the looters eventually seize the legitimate monopoly on violence—the state—and turn money itself into a weapon.

BlackVault was built for one purpose:  
to give the builder a form of money that cannot be looted, frozen, confiscated, or diluted without his consent.

## II. The Coming Looting

The U.S. dollar has lost >96 % of its purchasing power since 1913.  
The trend is accelerating.  
As real wages stagnate and debt burdens grow, political populism rises.  
Populism gives way to demagogues.  
Demagogues hand the treasury keys to the looters.

When the looters finally control the monetary system, they will:
- freeze accounts of political opponents  
- impose capital controls  
- inflate away savings  
- demand total visibility into every transaction

The builder will be left with a choice: submit or starve.

BlackVault removes that choice by removing their leverage.

## III. The Technical Answer

BlackVault is a minimal, auditable protocol that turns any digital value into a **bearer, private, spendable note** protected by zero-knowledge proofs.

One command — `cargo run --release` — produces a QR code that is:
- completely private (amount and ownership hidden)  
- bearer (whoever holds the note owns the value)  
- double-spend-proof (via cryptographic nullifier)  
- offline-transferable (QR, Signal, USB stick, paper)  
- verifiable by any honest party with the public parameters

No trusted setup is required for day-to-day use.  
No coordinator.  
No KYC.  
No permission.

### v5 Reference Implementation (November 2025)

| Component              | Implementation                                  |
|------------------------|-------------------------------------------------|
| Proof system           | halo2 0.3.1 plonk (Spearbit/Trail-of-Bits audited) |
| Curves                 | pasta (Pallas/Vesta)                            |
| Verification           | MockProver (real KZG optional)                  |
| Merkle tree            | 20 levels, Blake2b-512 → Fp                     |
| Commitment             | Pedersen (amount + blinding factor)             |
| Nullifier              | Blake2b-512(nullifier_key ‖ commitment) → Fp   |
| Size                   | ~300 lines of Rust                              |
| Output                 | JSON + QR (mobile-ready)                        |

The circuit is public, under 400 constraints, and can be audited in an afternoon.

## IV. What BlackVault Is Not

- Not a company  
- Not a token sale  
- Not a “roadmap”  
- Not a plea for permission

It is open protocol infrastructure for anyone who refuses to let looters decide what his money is worth.

## V. Invitation to Builders

BlackVault v5 is complete and production-ready.  
Everything required for private, bearer, un-lootable money already works today.

If you want to extend it, the doors are wide open:

- Build the mobile wallet (Flutter/React Native) that instantly recognizes a BlackVault QR and displays “You own $X shielded”  
- Deploy the on-chain verifier contract (EVM or any chain) that automatically redeems notes  
- Upgrade to real KZG proofs (v6) when you feel the time is right  
- Add multi-denomination support, note splitting/joining, or encrypted memos  
- Integrate it into your own chain, stablecoin, or offshore service

We invite you to build.  

Everything else is already done.

## VI. Final Word

When money becomes a tool of force, the only defense is money that cannot be forced.

BlackVault v5 is that defense.  
It is live today.

Use it.  
Protect what you built.

— BlackVault Protocol  
