import { PublicKey } from '@solana/web3.js';
const assert = require('assert')
const anchor = require('@project-serum/anchor')
const {SystemProgram} = anchor.web3

describe('rustsolana', async () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const connection = anchor.getProvider().connection;
  const program = anchor.workspace.Rustsolana

  let [fundPDA, _] = await PublicKey.findProgramAddress(
        [anchor.utils.bytes.utf8.encode("fund")],
        program.programId
      );

  it('Create a fund', async() => {
    await program.rpc.create( {
      accounts: {
        fund: fundPDA,
        founder: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: []
    })
    const account = await program.account.fund.fetch(fundPDA)
    assert.ok(account.founder.equals(provider.wallet.publicKey))
    const balance = await connection.getBalance(fundPDA);
    console.log("Fund account: ", balance);
  })

  it('Donate to fund', async() => {
    const donor = anchor.web3.Keypair.generate();
    const donation = 3000000000;
    let signature = await connection.requestAirdrop(donor.publicKey, donation);
    await connection.confirmTransaction(signature);

    const donorBefore = await connection.getBalance(donor.publicKey);
    const fundBefore = await connection.getBalance(fundPDA);
    console.log("Fund account afrer: ", fundBefore);

    await program.rpc.donate(new anchor.BN(donation), {
      accounts: {
        donor: donor.publicKey,
        fund: fundPDA,
        systemProgram: SystemProgram.programId
      },
      signers: [donor]
    })

    const donorAfter = await connection.getBalance(donor.publicKey);
    const fundAfter = await connection.getBalance(fundPDA);
    console.log("Fund account afrer: ", fundAfter);

    assert.ok(donorAfter === donorBefore - donation)
    assert.ok(fundAfter === fundBefore + donation )

  })

  it('Withdrawal from fund', async() => {
    const thief = anchor.web3.Keypair.generate();
    const meBefore = await connection.getBalance(provider.wallet.publicKey);
    const fundBefore = await connection.getBalance(fundPDA);
    const thiefBefore = await connection.getBalance(thief.publicKey);
    console.log("Fund account before: ", fundBefore);
    console.log("Funder account before: ", meBefore);
    console.log("Thief account before: ", thiefBefore);

    try {
      await program.rpc.withdraw({
      accounts: {
        founder: thief.publicKey,
        fund: fundPDA,
        systemProgram: SystemProgram.programId
      },
      signers: []
    })
    } catch (error) {
      console.log(error);
    }

    const meAfter = await connection.getBalance(provider.wallet.publicKey);
    const fundAfter = await connection.getBalance(fundPDA);
    const thiefAfter = await connection.getBalance(thief.publicKey);
    console.log("Fund account afrer: ", fundAfter);
    console.log("Funder account after: ", meAfter);
    console.log("Thief account after: ", thiefAfter);
  })

})
