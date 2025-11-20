
# Contributing to BlackVault

BlackVault is an experiment in sovereign cryptographic accounting.  
It is early, exploratory, and open.

If you want to learn, build, question, or improve the architecture — you are welcome here.

We value clarity, curiosity, and precision.  
**Move slow and correct** is our ethos.

Whether you are a seasoned cryptographer or touching zero-knowledge tooling for the first time, there is a place for you in this project.

## Principles

- **Sovereignty first.** Contributions should reinforce transparency, verifiability, and user-controlled trust.
- **No black boxes.** If a piece of code cannot be explained simply, it does not belong in the system yet.
- **Learn in the open.** Questions, confusions, and mistakes are expected. Document what you learn.
- **High signal. Low noise.** Every contribution, large or small, aims to make the system easier to understand.

## How to Get Started

1. **Star the repository**  
   This increases visibility and helps others discover the project.

2. **Read the BLACKPAPER**  
   The BLACKPAPER defines the philosophical and technical direction. Understanding the intent helps you contribute meaningfully.

3. **Install the environment**  
   Set up the recommended halo2 tooling and Rust workspace. The repo contains a `/setup/` guide.

4. **Try the starter circuits**  
   Start with the circuits in `/circuits/learning/`.  
   If you can run the arithmetic examples or modify them, you are already contributing.

5. **Join the discussion**  
   Conversations happen in GitHub Issues. Larger architectural discussions may move to Discord.

## Ways You Can Contribute Today

1. **Documentation**  
   BlackVault needs clear documentation for:
   - The circuit architecture
   - The accountant proof model
   - Commitment schemes
   - Ledger invariant rules  
   If you can explain something in plain language, write it down.

2. **Code Improvements**  
   Look for issues labeled `good first issue`, `needs clarification`, `spec gap`, or `open design question`.  
   These are ideal entry points.

3. **Testing and Verification**  
   Experience with Rust, ZK circuit testing, or property-based testing is highly valuable for verifying constraints and invariants.

4. **Research and Analysis**  
   We welcome contributions on:
   - Comparisons to CBDC or central-bank architectures
   - Zero-knowledge commitment models
   - Cryptographic accounting primitives
   - Formal verification approaches

5. **Architecture Discussions**  
   If you have strong opinions about protocol design, open an Issue titled `RFC: <topic>`.

## Contributing Workflow

1. Fork the repository
2. Create a new branch from `main`
3. Write clean and well-commented code
4. Run tests. Add new ones when appropriate
5. Open a Pull Request with a clear description of intent
6. Be open to feedback and discussion

No contribution is too small. Even identifying confusing parts of the code is valuable.

## Style Guide

- Keep code readable and intentional
- Prefer explicit names over abbreviations
- Keep functions short and single-purpose
- Circuits should be documented with diagrams or flow descriptions
- Avoid unnecessary abstractions
- No em-dashes anywhere in contributions

## Community Expectations

Be patient, be respectful, and stay mission-aligned.

BlackVault is an experiment, not a corporate product.  
We are here to discover what is possible when accounting becomes cryptographically verifiable and decentralized.

If you are unsure where to start, open an Issue titled **“Help me get started”** and we will guide you.

— BlackVault Protocol
