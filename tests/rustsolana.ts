import { PublicKey } from '@solana/web3.js';
import { BN } from 'bn.js';
const assert = require('assert')
const anchor = require('@project-serum/anchor')
const {SystemProgram} = anchor.web3

describe('rustsolana', async () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const connection = anchor.getProvider().connection;
  const program = anchor.workspace.Rustsolana

  let [fundPDA, _] = await PublicKey.findProgramAddress(
        [Buffer.from("fund")],
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
  })

  it('Donate to fund', async() => {
    const donor = anchor.web3.Keypair.generate();
    const donation = 3000000000;
    let signature = await connection.requestAirdrop(donor.publicKey, donation);
    await connection.confirmTransaction(signature);

    const accountBefore = await program.account.fund.fetch(fundPDA)

    let [donationPDA, _] = await PublicKey.findProgramAddress(
      [
        Buffer.from("donation"),
        accountBefore.donations.toBuffer('le', 8)
      ],
      program.programId
    );

    const donorBefore = await connection.getBalance(donor.publicKey);
    const fundBefore = await connection.getBalance(fundPDA);

    await program.rpc.donate(new anchor.BN(donation), {
      accounts: {
        donor: donor.publicKey,
        donation: donationPDA,
        fund: fundPDA,
        systemProgram: SystemProgram.programId
      },
      signers: [donor]
    })

    const donorAfter = await connection.getBalance(donor.publicKey);
    const fundAfter = await connection.getBalance(fundPDA);
    const rent = await connection.getBalance(donationPDA);

    assert.ok(donorAfter === donorBefore - donation)
    assert.ok(fundAfter === fundBefore + donation - rent)

    const accountAfter = await program.account.fund.fetch(fundPDA)
    assert.ok(accountAfter.donations.eq(new anchor.BN(1)))
  })

  it('Withdrawal from fund', async() => {

    const founderBefore = await connection.getBalance(provider.wallet.publicKey);
    const fundBefore = await connection.getBalance(fundPDA);

    await program.rpc.withdraw({
      accounts: {
        founder: provider.wallet.publicKey,
        fund: fundPDA,
        systemProgram: SystemProgram.programId
      },
      signers: []
    })

    const founderAfter = await connection.getBalance(provider.wallet.publicKey);
    const fundAfter = await connection.getBalance(fundPDA);

    assert.ok(fundAfter === 0)
    assert.ok(founderAfter === founderBefore + fundBefore)

  })

  it('Try to rob the fund', async() => {
    const thief = anchor.web3.Keypair.generate();
    const fundBefore = await connection.getBalance(fundPDA);
    const thiefBefore = await connection.getBalance(thief.publicKey);

    try {
      await program.rpc.withdraw({
      accounts: {
        founder: thief.publicKey,
        fund: fundPDA,
        systemProgram: SystemProgram.programId
      },
      signers: []
    }
    )
    assert.ok(false);
    } catch (error) {
      assert.ok(true);
    }

    const fundAfter = await connection.getBalance(fundPDA);
    const thiefAfter = await connection.getBalance(thief.publicKey);

    assert.ok(fundAfter === fundBefore)
    assert.ok(thiefAfter === thiefBefore)
  })

})
