import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { PdaAirdrop } from "../target/types/pda_airdrop";
import { expect } from "chai";
import { getMint, createAssociatedTokenAccount } from "@solana/spl-token";

describe("pda-airdrop", () => {
  const program = anchor.workspace.PdaAirdrop as Program<PdaAirdrop>;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const signer = provider.wallet as anchor.Wallet;

  it("can create airdrop account", async () => {
    await program.methods.createAirdop().rpc();

    const [mint] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("mint")],
      program.programId
    );

    const mintAccount = await getMint(provider.connection, mint);

    expect(mintAccount.isInitialized).eql(true);
  });

  it("can airdrop token to account", async () => {
    const [mint, _mintBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("mint")],
      program.programId
    );

    const ata = await createAssociatedTokenAccount(
      provider.connection,
      signer.payer,
      mint,
      signer.publicKey
    );

    const supply = new anchor.BN(200);
    await program.methods
      .executeAirdrop(supply)
      .accounts({
        ata,
      })
      .rpc();

    const mintAccount = await getMint(provider.connection, mint);

    expect(Number(mintAccount.supply)).equal(supply.toNumber());
  });
});
