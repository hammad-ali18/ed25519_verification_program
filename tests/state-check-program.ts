import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
// import { StateCheckProgram } from "../target/types/state_check_program";
import { readFileSync } from "fs";
import keccak256 from "keccak256";
import secp256k1 from "secp256k1";
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
  const programId = new anchor.web3.PublicKey("ExuXEXbXpYwNNVrgbMA6qCLruq9NH29zmUfCBrwXmPGj");
  const program = new Program(idl, programId, anchor.getProvider());

  let [maleAccount, bump] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("male_account")],
    programId
  );
  console.log("maleACcount",maleAccount.toBase58())

  it("Is initialized!", async () => {
    // Add your test here.
  let name="hammad"
  let age = 22
    const tx = await program.methods.setMaleData(name,age).accounts({
      maleAccount:maleAccount,
      user:walletKeyPair.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
      
    }).signers([walletKeyPair]).rpc();
    console.log("Your transaction signature", tx);
 
  });


  it("Verifies Signautre",async()=>{
    const message = "Hello, Blockchain";
    const messageHash = keccak256(Buffer.from(message));

    const {signature,recid} = secp256k1.ecdsaSign(
      Uint8Array.from(messageHash),
      walletKeyPair.secretKey.slice(0,32)
    );
    console.log("Signature:", Buffer.from(signature).toString('hex'));

    
    const expectedPubkey = secp256k1.publicKeyCreate(walletKeyPair.secretKey.slice(0, 32), true);

    console.log("message Hash Length:", messageHash.length);//32 byte
    console.log("signature length:", signature.length);//64 byte
    console.log("recovery ID Length:", recid);//1 bute

    console.log("Expected Public Key Length:", expectedPubkey.length);//33 bytes
    const tx = await program.methods
    .getMaleData(
     messageHash,      
      recid,                        
      Buffer.from(signature),        
      Buffer.from(expectedPubkey) 
    ).accounts({
      maleAccount:maleAccount
    })
    .rpc();

    
  console.log("Signature verification successful:", tx);
  })

  it("Verifies Signature will Fail- Worst Case Scenario", async () => {

    const randomKeypair =  anchor.web3.Keypair.generate();
    
    const message = "Hello, Blockchain";
    const messageHash = keccak256(Buffer.from(message));
  
    // Generate a valid signature
    const { signature, recid } = secp256k1.ecdsaSign(
        Uint8Array.from(messageHash),
        walletKeyPair.secretKey.slice(0, 32)
    );


    const expectedPubkey = secp256k1.publicKeyCreate(randomKeypair.secretKey.slice(0, 32), true);//using randomkeypair
    

    try {
      await program.methods
          .getMaleData(
            messageHash,      
            recid,                        
            Buffer.from(signature),        
            Buffer.from(expectedPubkey) 
          )
          .accounts({
              maleAccount: maleAccount
          })
          .rpc();

      assert.fail("Transaction should not have succeeded"); // This should not happen
  } catch (error) {
      assert.equal(error.error.errorCode, "SignatureVerificationFailed", "Expected SignatureVerificationFailed error");
      console.log("Caught expected error:", error.error);
  }});

});
