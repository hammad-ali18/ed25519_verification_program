use anchor_lang::prelude::*;
use solana_program::pubkey;
// use secp256k1_recover::secp256k1_recover;
// use sol_chainsaw::solana_sdk::{self, secp256k1_recover::secp256k1_recover};
// use solana_sdk::{entrypoint::ProgramResult, message, signature};
// use solana_program::entrypoint::ProgramResult;
// use solana_program::secp256k1_recover::secp256k1_recover;
// use ed25519_dalek::{PublicKey, Signature, Verifier}; // Add the required imports
use sha2::{Sha256, Sha512, Digest};
use solana_ed25519_instruction::{self};
use solana_program::{ed25519_program, instruction::Instruction, program::invoke};
use ed25519_dalek::{Signature, Verifier,KEYPAIR_LENGTH,PUBLIC_KEY_LENGTH};
use ed25519_dalek::VerifyingKey;
use borsh::{BorshSerialize,BorshDeserialize};
use solana_program::sysvar::instructions::{load_instruction_at_checked};
use hex;
declare_id!("7UpNbJTAT5jjni7qXeS1j4SfzP9JrnyaPyfzkMst3ZCr");
#[program]
pub mod state_check_program {






    use super::*;

    pub fn set_male_data(ctx: Context<MaleData>, name: String, age: u8) -> Result<()> {
        let male_account = &mut ctx.accounts.male_account;
        male_account.name = name.clone();
        male_account.age = age;
        male_account.married = true;

        msg!("set {} {}, married: {}", name, age, male_account.married);
        Ok(())
    }

    pub fn get_male_data(
        ctx: Context<GetMaleData>,
        _signature: Vec<u8>,          // Use Vec<u8> for the signature
        _message: Vec<u8>,              // Keep message as a byte slice
        _pubkey_from_sig: Vec<u8>,    // Use Vec<u8> for the public key
    ) -> Result<()> {
        let accounts = ctx.accounts;
        let _nonce_account = &mut accounts.nonce_account;
    

    let instruction_sysvar = &accounts.instruction_sysvar; // Assuming you have an `instruction_sysvar` account
    let verification_result = verify_ed25519_instruction(
        instruction_sysvar,
        &_pubkey_from_sig,   // Pass the public key
        &_message,           // Pass the message
        &_signature          // Pass the signature
    );
    verification_result.map_err(|_| Errorcode::SignatureVerificationFailed)?;

        let male_account = &accounts.male_account;
    
        msg!("Retrieved male account: Name: {}, Age: {}", male_account.name, male_account.age);
        Ok(())
    }
    
}

pub fn verify_ed25519_instruction(
    instruction_sysvar: &AccountInfo,  // Passed from sysvar
    expected_public_key: &[u8],        // Public key as a byte slice
    message: &[u8],                    // Message as a byte slice
    signature: &[u8]                   // Signature as a byte slice
) -> Result<()> {
    // Load the current instruction index
    msg!("hi 0");
    let current_index = load_current_index_checked(instruction_sysvar)?;
    if current_index == 0 {
        return Err(Errorcode::MissingEd25519Instruction.into());
    }
    msg!("hi 1");

    // Load the actual instruction at the current index
    let ed25519_instruction = load_instruction_at_checked((current_index - 1) as usize, instruction_sysvar)?;
    msg!("hi 2");

    // Extract instruction data and check its length
    let instruction_data = ed25519_instruction.data;
    if instruction_data.len() < 2 {
        return Err(Errorcode::InvalidEd25519Instruction.into());
    }
    msg!("hi 3");

    msg!("instruction_data {:?} ,at[0] {:?}",instruction_data,instruction_data[0]);
    // Check the number of signatures, should be exactly 1
    let num_signatures = instruction_data[0];
    if num_signatures != 1 {
        return Err(Errorcode::InvalidEd25519Instruction.into());
    }
    msg!("hi 4");


    // Parse Ed25519SignatureOffsets (signature, pubkey, message offsets)
    let offsets: Ed25519SignatureOffsets = Ed25519SignatureOffsets::try_from_slice(&instruction_data[2..16])?;
    msg!("hi 5");

    // Verify public key
    let pubkey_start = offsets.public_key_offset as usize;
    let pubkey_end = pubkey_start + 32;
    if &instruction_data[pubkey_start..pubkey_end] != expected_public_key {
        return Err(Errorcode::InvalidPublicKeyLength.into());
    }
    msg!("hi 6");

    // Verify message
    let msg_start = offsets.message_data_offset as usize;
    let msg_end = msg_start + offsets.message_data_size as usize;
    if &instruction_data[msg_start..msg_end] != message {
        return Err(Errorcode::InvalidMessage.into());
    }
    msg!("hi 7");

    // Verify signature
    let sig_start = offsets.signature_offset as usize;
    let sig_end = sig_start + 64;
    if &instruction_data[sig_start..sig_end] != signature {
        return Err(Errorcode::InvalidSignature.into());
    }

    msg!("hi 8");

    Ok(())
}

fn load_current_index_checked(instruction_sysvar: &AccountInfo) -> Result<u16> {
    let instruction_index = solana_program::sysvar::instructions::load_current_index_checked(instruction_sysvar)?;
    Ok(instruction_index as u16)
}  
    // Helper function to load the current instruction index from sysvar
    


#[derive(Accounts)]
pub struct MaleData<'info> {
    #[account(
        init_if_needed,
        payer = user, 
        space = 8 + std::mem::size_of::<Male>(),
        seeds=[b"male_account",user.key().as_ref()],
        bump
    )]
    pub male_account: Account<'info, Male>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts,BorshDeserialize, BorshSerialize)]
pub struct GetMaleData<'info> {
    #[account(mut)]
    pub male_account: Account<'info, Male>,
    #[account(
        init_if_needed,
        payer = user, 
        space = 8 + std::mem::size_of::<Nonce>(),
        seeds=[b"nonce_account",user.key().as_ref()],
        bump
)]
    pub nonce_account: Account<'info, Nonce>,
    ///CHECK:
    pub instruction_sysvar: AccountInfo<'info>,   // Add this to access instruction sysvar

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,


}
#[derive(AnchorSerialize, AnchorDeserialize)]
struct Ed25519SignatureOffsets {
    signature_offset: u16,
    signature_instruction_index: u16,
    public_key_offset: u16,
    public_key_instruction_index: u16,
    message_data_offset: u16,
    message_data_size: u16,
    message_instruction_index: u16,
}

#[account]
#[derive(Default)]

pub struct Nonce {
    pub last_nonce: u64,
}
#[account]
#[derive(Default, InitSpace)]
pub struct Male {
    #[max_len(20)]
    pub name: String,
    pub age: u8,
    pub married: bool,
}

#[error_code]
pub enum Errorcode {
    #[msg("Error in recovery process")]
    ErrorInRecovery,
    #[msg("Invalid Signature Length.")]
    InvalidSignatureLength,

    #[msg("Signature verification failed.")]
    SignatureVerificationFailed,
    #[msg("Signature verification failed.")]
    InvalidPublicKeyLength,
    #[msg("Nonce already used or invalid.")]
    NonceAlreadyUsed,
    #[msg("Nonce already used or invalid.")]
    InvalidNonce,

    MissingEd25519Instruction,
    InvalidEd25519Instruction,
    InvalidMessage,
    InvalidSignature


}

// pub fn verifyable(
//     user: &Signer<'_>,
//     signature: Vec<u8>,
//     pubkey_from_sig:Pubkey,
//     nonce: u64,
//     nonce_account: &mut Nonce,
// )-> Result<()>{

//     if nonce <= nonce_account.last_nonce {
//         return err!(ErrorCode::NonceAlreadyUsed);
//     }
//     require_eq!(nonce , nonce_account.last_nonce +1,ErrorCode::InvalidNonce);

//     if signature.len() != 64 {
//         return err!(ErrorCode::InvalidSignatureLength);
//     }
    
//     if pubkey_from_sig != user.key() {
//         return err!(ErrorCode::SignatureVerificationFailed); // Or another suitable error
//     }

//     nonce_account.last_nonce = nonce;
 
//  Ok(())
// }

//helper function
// pub fn verify_signature(
//     // message_hash: &[u8],    // hashed message (typically keccak256)
//     // recovery_id: u8,        // Recovery ID (0 or 1)
//     // signature: &[u8],       // Secp256k1 signature (64 bytes)
//     // expected_pubkey: &[u8], // Compressed public key (33 bytes)
//     // nonce: u64,
//     message: Vec<u8>, // Now take the original message instead of the hashed version
//     signature: Vec<u8>,
//     expected_pubkey: Vec<u8>,
//     nonce: u64,
//     nonce_account: &mut Nonce,
// ) -> Result<()> {

//     if nonce <= nonce_account.last_nonce {
//         return err!(ErrorCode::NonceAlreadyUsed);
//     }
//     if signature.len() != 64 {
//         return err!(ErrorCode::InvalidSignatureLength);
//     }

//     require_eq!(nonce , nonce_account.last_nonce +1,ErrorCode::InvalidNonce);


    

//     //new 
//     // let mut hasher = Sha512::new();
//     // hasher.update(&message);
//     // let message_hash = hasher.finalize();
//     // let pubkey = PublicKey::from_bytes(&expected_pubkey).map_err(|_|ErrorCode::InvalidPublicKeyLength)?;
//     // let signature = Signature::from_bytes(&signature).map_err(|_| ErrorCode::InvalidSignatureLength)?;

//     // pubkey.verify(&message_hash, &signature).map_err(|_| ErrorCode::SignatureVerificationFailed)?;
//     // If verification is successful, update the nonce
//     nonce_account.last_nonce = nonce;
 
//  Ok(())
    
// //     msg!("get {} {}", male_account.name, male_account.age);
// //     let recovered_pubkey = secp256k1_recover(message_hash, recovery_id, signature)
// //         .map_err(|_| Error::ErrorInRecovery)?;

// // msg!("recovered pbkey {:?} {:?}",recovered_pubkey.0,recovered_pubkey.to_bytes());
// //     let recovered_pubkey_bytes = recovered_pubkey.to_bytes();

// //     let mut compressed = [0u8; 33];

// //     //check length
// //     match recovered_pubkey_bytes.len() {
// //         64 => {
// //             compressed[0] = if recovery_id % 2 == 0 { 0x03 } else { 0x02 };
// //             compressed[1..].copy_from_slice(&recovered_pubkey_bytes[0..32]); // use the first 32 bytes
// //         }
// //         65 => {
// //             // performing compression

// //             // Handle uncompressed public key throught prefixes
// //             compressed[0] = if recovered_pubkey_bytes[63] % 2 == 0 {
// //                 0x03
// //             } else {
// //                 0x02
// //             };
// //             compressed[1..].copy_from_slice(&recovered_pubkey_bytes[1..33]); // Use the first 32 bytes (1 to 32)
// //         }
// //         _ => {
// //             msg!(
// //                 "Unexpected recovered public key length: {}",
// //                 recovered_pubkey_bytes.len()
// //             );
// //             return err!(Error::InvalidPublicKeyLength);
// //         }
// //     }

// //     msg!("compressed {:?} and expected {:?}",compressed,expected_pubkey);
// //     // Compare the compressed public key with the expected public key
// //     if compressed == expected_pubkey {
// //         msg!("Signature verification successful");
// //         nonce_account.last_nonce = nonce;
// //         Ok(())
// //     } else {
// //         msg!("Signature verification failed");
// //         msg!(
// //             "compressed & expected {}, {}, {:?}, {:?},",
// //             compressed.len(),
// //             expected_pubkey.len(),
// //             compressed,
// //             expected_pubkey
// //         );
// //         err!(Error::SignatureVerificationFailed)
// //     }
// }
