import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Infinityid } from "../target/types/infinityid";
import { PublicKey } from "@solana/web3.js";

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Infinityid as Program<Infinityid>;
  const provider = program.provider;
  const programId = program.programId;

  const user = anchor.web3.Keypair.generate();

describe("infinityid", () => {

  
  it("create profile!", async () => {
    // Add your test here.
    const createProfileParams = {
      username: "Marcus"
    };
    const tx = await program.methods
      .createProfile(createProfileParams)
      .accounts({
        user: provider.publicKey,
        profile: getProfilePda(provider.publicKey)
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("earn points!", async () => {
    // Add your test here.
    const earnPointsParams = {
      points: new BN(10)
    };
    const tx = await program.methods
      .earnPoints(earnPointsParams)
      .accounts({
        user: provider.publicKey,
        profile: getProfilePda(provider.publicKey)
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});

const getProfilePda = (authority: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("create_profile"), authority.toBuffer()],
    programId
  )[0]
}