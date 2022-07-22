import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { PdaAirdrop } from "../target/types/pda_airdrop";
import { expect } from "chai";
import {
  getMint,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("pda-airdrop", () => {
  const program = anchor.workspace.PdaAirdrop as Program<PdaAirdrop>;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const signer = provider.wallet as anchor.Wallet;

  // Read this https://github.com/coral-xyz/anchor/blob/e606e5a0724bc6ac52b20ca63d8a2a912aca22bd/docs/src/pages/docs/pdas.md

  it("can create airdrop account", async () => {
    // const mint = web3.Keypair.generate();

    // const [airdrop, bump] = await anchor.web3.PublicKey.findProgramAddress(
    //   [Buffer.from("airdrop")],
    //   program.programId
    // );

    await program.methods
      .createAirdop()
      // .accounts({
      // signer: signer.publicKey,
      // airdrop,
      // mint: mint.publicKey,
      // tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      // rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      // systemProgram: anchor.web3.SystemProgram.programId,
      // })
      // .signers([mint])
      .rpc();

    // const [airdrop] = await web3.PublicKey.findProgramAddress(
    //   [Buffer.from("airdrop")],
    //   program.programId
    // );

    // const airdropAccount = await program.account.airdrop.fetch(airdrop);
    // const mint = airdropAccount.mint;
    // const mintAccount = await getMint(provider.connection, mint);

    // expect(mintAccount.isInitialized).eql(true);
  });

  it("queue can be created", async () => {
    const [queue] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("queue")],
      program.programId
    );

    await program.methods
      .createQueue()
      // .accounts({
      //   queue,
      // })
      .rpc();

    const fetchedQueue = await program.account.queue.fetch(queue);

    expect(fetchedQueue.size).equal(0);
  });

  it("can airdrop token to account", async () => {
    const [airdrop, airdropBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("airdrop")],
      program.programId
    );

    // const airdropAccount = await program.account.airdrop.fetch(airdrop);
    // const mint = airdropAccount.mint;

    const [mint, mintBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("mint")],
      program.programId
    );

    // const ata = await getAssociatedTokenAddress(
    //   mint,
    //   provider.wallet.publicKey
    // );

    const ata = web3.Keypair.generate();

    const supply = new anchor.BN(200);
    await program.methods
      .executeAirdrop(supply, airdropBump, mintBump)
      .accounts({
        // signer: provider.wallet.publicKey,
        // airdrop,
        // mint,
        ata: ata.publicKey,
        // associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        // tokenProgram: TOKEN_PROGRAM_ID,
        // rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        // systemProgram: web3.SystemProgram.programId,
      })
      .signers([ata])
      .rpc();

    // const mintAccount = await getMint(provider.connection, mint);

    // expect(mintAccount.supply).equal(supply);
  });
});
