import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { PdaAirdrop } from "../target/types/pda_airdrop";
import { expect } from "chai";
import { getMint, getAssociatedTokenAddress } from "@solana/spl-token";

describe("pda-airdrop", () => {
  const program = anchor.workspace.PdaAirdrop as Program<PdaAirdrop>;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("can create airdrop account", async () => {
    const [mint, _bump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("mint")],
      program.programId
    );

    await program.methods
      .createAirdop()
      .accounts({
        mint,
      })
      .rpc();

    const mintAccount = await getMint(provider.connection, mint);

    expect(mintAccount.isInitialized).eql(true);
  });

  it("can airdrop token to account", async () => {
    const [mint, _bump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("mint")],
      program.programId
    );

    const ata = await getAssociatedTokenAddress(
      mint,
      provider.wallet.publicKey
    );

    const supply = new anchor.BN(200);
    await program.methods
      .executeAirdrop(supply)
      .accounts({
        mint,
        ata,
      })
      .rpc();

    const mintAccount = await getMint(provider.connection, mint);

    expect(mintAccount.supply).equal(supply);
  });
});
