import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Nft } from "../target/types/nft";

describe("nft", () => {
  
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Nft as Program<Nft>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
