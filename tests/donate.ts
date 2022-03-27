import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Donate } from "../target/types/donate";

describe("donate", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Donate as Program<Donate>;

  it("donates lamports to program's account", async () => {
    const LAMPORTS_TO_DONATE = new anchor.BN(100);
    const donater = anchor.web3.Keypair.generate();

    await program.rpc.donateProgram(LAMPORTS_TO_DONATE, {
      accounts: {
        sender: donater.publicKey,
        programAccount: program.programId,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [donater]
    })

    const programAccount = await program.account.programAccount.fetch(program.programId);
  	console.log(programAccount);
  })
});
