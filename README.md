# Recursion Example

This example demonstrates recursive zero-knowledge proofs in the RISC Zero zkVM. Each proof in the chain verifies the previous proof, creating a sequence of verified computations.

## Quick Start

```bash
RUST_LOG=info cargo run
```

#### Composition

The implementation uses RISC Zero's composition API to achieve recursion. See our [composition example](https://github.com/risc0/risc0/tree/main/examples/composition) for more information.