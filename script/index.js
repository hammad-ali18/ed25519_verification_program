// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { StateCheckProgram } from "../target/types/state_check_program";
// import { readFileSync } from "fs";

const anchor = require("@coral-xyz/anchor");
const { Program } = require("@coral-xyz/anchor");

const  {clusterApiUrl}=require("@solana/web3.js")
const { readFileSync } = require("fs");

const secretKey = Uint8Array.from(JSON.parse(readFileSync('/home/muhammad/wallet/keypair1.json', 'utf-8')));
const walletKeyPair = anchor.web3.Keypair.fromSecretKey(secretKey);

async function main(){
 // Configure the client to use the local cluster.
 const wallet = new anchor.Wallet(walletKeyPair);

 const connection = new anchor.web3.Connection(clusterApiUrl("devnet"), "confirmed");

 const provider = new anchor.AnchorProvider(connection, wallet, { preflightCommitment: "confirmed" });
 anchor.setProvider(provider);
//  const provider = anchor.AnchorProvider.env();
//  anchor.setProvider(provider);

 // const program = anchor.workspace.StateCheckProgram as Program<StateCheckProgram>;
 const idl = JSON.parse(readFileSync('target/idl/state_check_program.json','utf-8'));
 const programId = new anchor.web3.PublicKey("GaJkAURRGeFo8kPRX84KXYRukLvrtPpNvK7cXUYkm6nf");
 const program = new Program(idl, programId, anchor.getProvider());

 let [male, bump] =  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("male_account")],
    programId
  );
  
  //  const tx = await program.methods.getMaleData().accounts({
  //     male:male
  //   }).signers([walletKeyPair]).rpc();
  //   console.log("Your transaction signature", tx);


  const fetchData = await program.account.male.fetch(male);
  console.log("fetch #1", fetchData);

  let name="ahmed"
  let age = 30
    const tx1 = await program.methods.setMaleData(name,age).accounts({
      maleAccount:male,
      user:walletKeyPair.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
      
    }).signers([walletKeyPair]).rpc();
    console.log("Your transaction signature", tx1);

    const fetchDataAgain = await program.account.male.fetch(male);
    console.log("fetch #2", fetchDataAgain);
 

}

main()