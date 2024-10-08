import * as anchor from "@coral-xyz/anchor";
import * as new_anchor from "@project-serum/anchor";
import { Program ,BN} from "@coral-xyz/anchor";
import { Connection, Ed25519Program, Keypair, NonceAccount, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
// import { StateCheckProgram } from "../target/types/state_check_program";
import { readFileSync } from "fs";
import keccak256 from "keccak256";
import secp256k1 from "secp256k1";
import bs58 from 'bs58';
import { Signer } from 'ed25519-supercop';
import { Ed25519Keypair } from '@solana/web3.js';
import nacl from 'tweetnacl';
// const secp256k1 = require("s")
const secretKey = Uint8Array.from(JSON.parse(readFileSync('/home/muhammad/wallet/keypair1.json', 'utf-8')));
const walletKeyPair = anchor.web3.Keypair.fromSecretKey(secretKey);
describe("state-check-program",async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // const program = anchor.workspace.StateCheckProgram as Program<StateCheckProgram>;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const idl = JSON.parse(readFileSync('target/idl/state_check_program.json','utf-8'));
  const programId = new anchor.web3.PublicKey("4vegiiytVNFYtxwSZQ6KFi1P2ZyixmkYmsAfaSJeCkvY");
  const program = new Program(idl, programId, anchor.getProvider());

  let [maleAccount, bump] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("male_account"),walletKeyPair.publicKey.toBuffer()],
    programId
  );
  
  console.log("maleAccount",maleAccount.toBase58())

  let [nonceAccount,nonceBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("nonce_account"),walletKeyPair.publicKey.toBuffer()],
    programId
  )

  // it("Is initialized!", async () => {
  //   // Add your test here.
  // let name="hammad"
  // let age = 22
  //   const tx = await program.methods.setMaleData(name,age).accounts({
  //     maleAccount:maleAccount,
  //     user:walletKeyPair.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId
      
  //   }).signers([walletKeyPair]).rpc();
  //   console.log("Your transaction signature", tx);
 
  // });


  it("Verifies Signautre",async()=>{
    const message = "To avoid digital dognappers, sign below to authenticate with CryptoCorgis";
    // const messageHash = keccak256(Buffer.from(message));
    const encodedMessage = new TextEncoder().encode(message);

    console.log("messageHash",Buffer.from(encodedMessage).toString('hex'))
    const signature= nacl.sign.detached(encodedMessage,walletKeyPair.secretKey);


    console.log("Buffer Signature: ", Buffer.from(signature).toString('hex'));
    console.log("Signature & encoded Message",signature,encodedMessage)
    const nonce = new BN(3);
const ed25519Instruction = Ed25519Program.createInstructionWithPublicKey({
  publicKey:walletKeyPair.publicKey.toBytes(),
  message: encodedMessage,
  signature:signature
});
console.log("ed25519Instruction",ed25519Instruction);
 
   console.log(walletKeyPair.publicKey.toBase58())
    const tx = await program.methods
    .getMaleData(
      Buffer.from(signature),        
      Buffer.from(encodedMessage),      
      Buffer.from(walletKeyPair.publicKey.toBytes()),
      nonce
    ).accounts({
      maleAccount:maleAccount,
      nonceAccount:nonceAccount,
      user: walletKeyPair.publicKey,
      SystemProgram: anchor.web3.SystemProgram.programId,
      instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
    }).preInstructions([ed25519Instruction]).signers([walletKeyPair])
    .rpc();

    
  console.log("Signature verification successful:", tx);
  })

  // it("Verifies Signature will Fail- Worst Case Scenario", async () => {

  //   const randomKeypair =  anchor.web3.Keypair.generate();
    
  //   const message = "Hello, Blockchain";
  //   const messageHash = keccak256(Buffer.from(message));
  
  //   // Generate a valid signature
  //   const { signature, recid } = secp256k1.ecdsaSign(
  //       Uint8Array.from(messageHash),
  //       walletKeyPair.secretKey.slice(0, 32)
  //   );


  //   const expectedPubkey = secp256k1.publicKeyCreate(randomKeypair.secretKey.slice(0, 32), true);//using randomkeypair
    

  //   try {
  //     await program.methods
  //         .getMaleData(
  //           messageHash,      
  //           recid,                        
  //           Buffer.from(signature),        
  //           Buffer.from(expectedPubkey) 
  //         )
  //         .accounts({
  //             maleAccount: maleAccount
  //         })
  //         .rpc();

  //     assert.fail("Transaction should not have succeeded"); // This should not happen
  // } catch (error) {
  //     assert.equal(error.error.errorCode, "SignatureVerificationFailed", "Expected SignatureVerificationFailed error");
  //     console.log("Caught expected error:", error.error);
  // }});

});
