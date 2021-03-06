import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import assert from "assert";

import { PoolProgram } from "../target/types/pool_program";

describe("pool-program", () => {
  anchor.setProvider(anchor.Provider.env());

  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  const program = anchor.workspace.PoolProgram as Program<PoolProgram>;

  const charactersProgram = new anchor.web3.PublicKey(
    "HYx4J9Np2BJTgibvYjNvfKhXk9zsxyLVDDskRDbE64yg"
  );

  // PDA generated by the characters-program.
  const user = new anchor.web3.PublicKey(
    "5rEfo4VjDeGoTvsSwcu9ggNtQfq3KcLcJ5VFbmnkptZQ"
  );

  const characterPoolSeed = Buffer.from("character_pool");

  let characterPool: anchor.web3.PublicKey;
  anchor.web3.PublicKey.findProgramAddress(
    [characterPoolSeed],
    program.programId
  ).then((val) => {
    characterPool = val[0];
  });

  it("Initializes the pool.", async () => {
    const tx = await program.rpc.initialize(
      {
        capacity: 10,
        seed: characterPoolSeed,
      },
      {
        accounts: {
          state: characterPool,
          authority: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      }
    );

    console.log("Pool account:", characterPool.toBase58());
    console.log("Your transaction signature:", tx);
  });

  it("Pushes a new member to the pool.", async () => {
    await program.rpc.add({
      accounts: {
        state: characterPool,
        userAccount: user,
        authority: program.provider.wallet.publicKey,
        charactersProgram,
      },
    });

    const state = await program.account.programState.fetch(characterPool);

    assert.deepEqual(state.members.length, 1);
    assert.deepEqual(state.members[0], user);
  });

  it("Takes a member from the pool.", async () => {
    await program.rpc.remove({
      accounts: {
        state: characterPool,
        userAccount: user,
        authority: program.provider.wallet.publicKey,
      },
    });

    const state = await program.account.programState.fetch(characterPool);

    assert.deepEqual(state.members.length, 0);
  });

  it("Closes the pool.", async () => {
    await program.rpc.close({
      accounts: {
        state: characterPool,
        authority: program.provider.wallet.publicKey,
      },
    });

    try {
      await program.account.programState.fetch(characterPool);
      assert(false);
    } catch (err) {
      assert(err.toString().includes("Account does not exist"));
    }
  });
});
