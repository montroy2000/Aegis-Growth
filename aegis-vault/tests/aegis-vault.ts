import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AegisVault } from "../target/types/aegis_vault";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo } from "@solana/spl-token";
import { assert } from "chai";

describe("aegis-vault", () => {
    // Configure the client to use the local cluster
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.AegisVault as Program<AegisVault>;

    let usdcMint: PublicKey;
    let shareMint: PublicKey;
    let vaultPda: PublicKey;
    let vaultBump: number;
    let vaultUsdcPda: PublicKey;
    let userUsdcAccount: PublicKey;
    let userSharesAccount: PublicKey;
    let userPositionPda: PublicKey;

    // Mock oracle accounts (in production, use real Pyth/Switchboard feeds)
    let pythFeed: Keypair;
    let switchboardFeed: Keypair;

    before(async () => {
        // Create USDC mint
        usdcMint = await createMint(
            provider.connection,
            provider.wallet.payer,
            provider.wallet.publicKey,
            null,
            6 // USDC has 6 decimals
        );

        // Create user USDC account and mint some tokens
        userUsdcAccount = await createAccount(
            provider.connection,
            provider.wallet.payer,
            usdcMint,
            provider.wallet.publicKey
        );

        await mintTo(
            provider.connection,
            provider.wallet.payer,
            usdcMint,
            userUsdcAccount,
            provider.wallet.publicKey,
            10_000_000_000 // 10,000 USDC
        );

        // Derive PDAs
        [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
            [Buffer.from("vault")],
            program.programId
        );

        [vaultUsdcPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("vault-usdc")],
            program.programId
        );

        [userPositionPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user-position"), provider.wallet.publicKey.toBuffer()],
            program.programId
        );

        // Create mock oracle accounts
        pythFeed = Keypair.generate();
        switchboardFeed = Keypair.generate();
    });

    describe("Initialization", () => {
        it("Initializes the vault", async () => {
            const maxLeverageBps = 15000; // 1.50x
            const hfFloorBps = 24000; // 2.40

            const tx = await program.methods
                .initializeVault(maxLeverageBps, hfFloorBps)
                .accounts({
                    vault: vaultPda,
                    authority: provider.wallet.publicKey,
                    usdcMint: usdcMint,
                    vaultUsdc: vaultUsdcPda,
                    pythUsdcFeed: pythFeed.publicKey,
                    switchboardUsdcFeed: switchboardFeed.publicKey,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .rpc();

            const vault = await program.account.vault.fetch(vaultPda);

            assert.equal(vault.maxLeverageBps, maxLeverageBps);
            assert.equal(vault.hfFloorBps, hfFloorBps);
            assert.equal(vault.totalSupplied.toNumber(), 0);
            assert.equal(vault.totalBorrowed.toNumber(), 0);
            assert.equal(vault.totalShares.toNumber(), 0);
            assert.equal(vault.oracleStaleSlots.toNumber(), 150);
            assert.equal(vault.pegWarnBps, 10);
            assert.equal(vault.pegExitBps, 25);
            assert.equal(vault.pegPanicBps, 50);
            assert.equal(vault.cooldownSlots.toNumber(), 30000);
            assert.equal(vault.reexpansionDelaySec.toNumber(), 30000);
        });

        it("Rejects invalid leverage parameters", async () => {
            try {
                await program.methods
                    .initializeVault(25000, 24000) // 2.5x leverage (too high)
                    .accounts({
                        vault: vaultPda,
                        authority: provider.wallet.publicKey,
                        usdcMint: usdcMint,
                        vaultUsdc: vaultUsdcPda,
                        pythUsdcFeed: pythFeed.publicKey,
                        switchboardUsdcFeed: switchboardFeed.publicKey,
                        systemProgram: SystemProgram.programId,
                        tokenProgram: TOKEN_PROGRAM_ID,
                    })
                    .rpc();

                assert.fail("Should have rejected invalid leverage");
            } catch (err) {
                assert.include(err.message, "InvalidLeverage");
            }
        });
    });

    describe("Deposits", () => {
        it("Handles first deposit (1:1 shares)", async () => {
            const depositAmount = new anchor.BN(1000_000_000); // 1000 USDC

            const tx = await program.methods
                .deposit(depositAmount)
                .accounts({
                    vault: vaultPda,
                    userPosition: userPositionPda,
                    user: provider.wallet.publicKey,
                    userUsdc: userUsdcAccount,
                    vaultUsdc: vaultUsdcPda,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .rpc();

            const userPosition = await program.account.userPosition.fetch(userPositionPda);
            const vault = await program.account.vault.fetch(vaultPda);

            assert.equal(userPosition.shares.toNumber(), depositAmount.toNumber());
            assert.equal(vault.totalShares.toNumber(), depositAmount.toNumber());
        });

        it("Handles subsequent deposits with correct share calculation", async () => {
            const depositAmount = new anchor.BN(500_000_000); // 500 USDC

            const vaultBefore = await program.account.vault.fetch(vaultPda);
            const equity = vaultBefore.totalSupplied.sub(vaultBefore.totalBorrowed);
            const expectedShares = depositAmount.mul(vaultBefore.totalShares).div(equity);

            await program.methods
                .deposit(depositAmount)
                .accounts({
                    vault: vaultPda,
                    userPosition: userPositionPda,
                    user: provider.wallet.publicKey,
                    userUsdc: userUsdcAccount,
                    vaultUsdc: vaultUsdcPda,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .rpc();

            const userPosition = await program.account.userPosition.fetch(userPositionPda);

            // Shares should be proportional to equity
            assert.approximately(
                userPosition.shares.toNumber(),
                expectedShares.toNumber(),
                1 // Allow 1 lamport rounding error
            );
        });
    });

    describe("Withdrawals", () => {
        it("Handles withdrawal with share burning", async () => {
            const userPositionBefore = await program.account.userPosition.fetch(userPositionPda);
            const withdrawShares = new anchor.BN(250_000_000); // 250 shares

            await program.methods
                .withdraw(withdrawShares)
                .accounts({
                    vault: vaultPda,
                    userPosition: userPositionPda,
                    user: provider.wallet.publicKey,
                    userUsdc: userUsdcAccount,
                    vaultUsdc: vaultUsdcPda,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .rpc();

            const userPositionAfter = await program.account.userPosition.fetch(userPositionPda);

            assert.equal(
                userPositionAfter.shares.toNumber(),
                userPositionBefore.shares.sub(withdrawShares).toNumber()
            );
        });

        it("Rejects withdrawal exceeding shares", async () => {
            const userPosition = await program.account.userPosition.fetch(userPositionPda);
            const excessiveShares = userPosition.shares.add(new anchor.BN(1));

            try {
                await program.methods
                    .withdraw(excessiveShares)
                    .accounts({
                        vault: vaultPda,
                        userPosition: userPositionPda,
                        user: provider.wallet.publicKey,
                        userUsdc: userUsdcAccount,
                        vaultUsdc: vaultUsdcPda,
                        tokenProgram: TOKEN_PROGRAM_ID,
                    })
                    .rpc();

                assert.fail("Should have rejected excessive withdrawal");
            } catch (err) {
                assert.include(err.message, "InsufficientEquity");
            }
        });
    });

    describe("State Machine", () => {
        // Note: These tests would require mocking oracle data
        // In production, use a test harness that can inject oracle prices

        it("Should determine LOOP state when healthy", async () => {
            // Mock: peg at $1.00, HF at 3.12
            // Expected: LOOP state
        });

        it("Should determine CONTRACT state on warning", async () => {
            // Mock: peg at $0.989 (11 bps deviation)
            // Expected: CONTRACT state
        });

        it("Should determine EXIT state on critical depeg", async () => {
            // Mock: peg at $0.974 (26 bps deviation)
            // Expected: EXIT state
        });

        it("Should determine PANIC state on oracle failure", async () => {
            // Mock: stale oracle (>150 slots)
            // Expected: PANIC state
        });
    });

    describe("Rebalancing", () => {
        it("Enforces cooldown period", async () => {
            // First rebalance
            await program.methods
                .rebalance()
                .accounts({
                    vault: vaultPda,
                    keeper: provider.wallet.publicKey,
                    pythFeed: pythFeed.publicKey,
                    switchboardFeed: switchboardFeed.publicKey,
                })
                .rpc();

            // Try immediate rebalance (should fail)
            try {
                await program.methods
                    .rebalance()
                    .accounts({
                        vault: vaultPda,
                        keeper: provider.wallet.publicKey,
                        pythFeed: pythFeed.publicKey,
                        switchboardFeed: switchboardFeed.publicKey,
                    })
                    .rpc();

                assert.fail("Should have enforced cooldown");
            } catch (err) {
                assert.include(err.message, "RebalanceCooldown");
            }
        });
    });
});
