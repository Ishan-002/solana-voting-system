import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaVotingSystem } from "../target/types/solana_voting_system";

describe("solana-voting-system", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaVotingSystem as Program<SolanaVotingSystem>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
