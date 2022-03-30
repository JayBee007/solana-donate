import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, SystemProgram } from '@solana/web3.js';

import { Donate } from "../target/types/donate";

describe("donate", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Donate as Program<Donate>;

  it("creates a PDA account for current wallet holder", async () => {
    const [userDonationPDA, _] = await PublicKey.findProgramAddress([
      anchor.utils.bytes.utf8.encode("user-donation"),
      anchor.getProvider().wallet.publicKey.toBuffer()
    ], program.programId)

    console.log("program.programId", program.programId.toBase58())
    console.log('userDonationPDA', userDonationPDA.toBase58())

    await program.rpc.createProgramAddress({
        accounts:
          {
            sender: anchor.getProvider().wallet.publicKey,
            userDonation: userDonationPDA,
            systemProgram: SystemProgram.programId
         },
  })

    console.log('=>', await program.account.userDonation.fetch(userDonationPDA))
  })
});
