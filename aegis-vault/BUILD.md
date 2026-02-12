# Smart Contract Build Instructions

## You're in the Smart Contract Directory!

This directory uses **Anchor** (not npm) to build the Solana program.

---

## Quick Build

```bash
# 1. Load Cargo environment
source $HOME/.cargo/env

# 2. Build the program
./scripts/build.sh
```

---

## Available Commands

### Build Program
```bash
./scripts/build.sh
```

### Run Tests
```bash
./scripts/test.sh
# OR
anchor test
```

### Deploy to Devnet
```bash
./scripts/deploy-devnet.sh
```

---

## ❌ These Don't Work Here

- `npm run dev` - Not for smart contracts
- `npm run build` - Not for smart contracts
- `npm start` - Not for smart contracts

The only npm command that works is:
- `npm test` (runs `anchor test`)

---

## For the Frontend

If you want to run the **frontend** (Next.js app):

```bash
# Go to frontend directory
cd ../aegis-app

# Update Node.js first
nvm install 20
nvm use 20

# Run dev server
npm run dev
```

---

## Need Help?

See the main README or run:
```bash
./scripts/build.sh --help
```
