# Solana Development Environment Setup

## Prerequisites

To build and deploy the Aegis Stable Carry smart contract, you need the following tools installed:

### 1. Rust Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Solana CLI

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

### 3. Anchor Framework

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### 4. Verify Installation

```bash
rustc --version
solana --version
anchor --version
```

## Next Steps

Once the environment is set up, run:

```bash
cd /Users/melvicsmith/Aegis-Growth
anchor init aegis-vault
cd aegis-vault
anchor build
```

This will create the Anchor project structure and compile the program.
