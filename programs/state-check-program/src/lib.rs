
use anchor_lang::prelude::*;
// use secp256k1_recover::secp256k1_recover;
// use sol_chainsaw::solana_sdk::{self, secp256k1_recover::secp256k1_recover};
// use solana_sdk::{entrypoint::ProgramResult, message, signature};
use solana_program::secp256k1_recover::secp256k1_recover;
use solana_program::entrypoint::ProgramResult;
declare_id!("ExuXEXbXpYwNNVrgbMA6qCLruq9NH29zmUfCBrwXmPGj");

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

    pub fn get_male_data(ctx: Context<GetMaleData>,
        message_hash: Vec<u8>,    
        recovery_id: u8,          
        signature: Vec<u8>,       
        expected_pubkey: Vec<u8>,) -> Result<()> {

        //verify signature
        verify_signature(&message_hash, recovery_id, &signature, &expected_pubkey)?;

        let male_account = &ctx.accounts.male_account;
        msg!("get {} {}", male_account.name, male_account.age);
        Ok(())
    }

   
}

#[derive(Accounts)]
pub struct MaleData<'info> {
    #[account(mut)] 
    pub male_account: Account<'info, Male>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetMaleData<'info> {
    #[account(mut)]
    pub male_account: Account<'info, Male>,
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
pub enum Error {

    #[msg("Error in recovery process")]
    ErrorInRecovery,
    #[msg("Invalid Signature Length.")]
    InvalidSignatureLength,
    
    #[msg("Signature verification failed.")]
    SignatureVerificationFailed,
    #[msg("Signature verification failed.")]
    InvalidPublicKeyLength,
    
    
}

//helper function
pub fn verify_signature(
    message_hash: &[u8],     // hashed message (typically keccak256)
    recovery_id: u8,         // Recovery ID (0 or 1)
    signature: &[u8],        // Secp256k1 signature (64 bytes)
    expected_pubkey: &[u8],  // Compressed public key (33 bytes)
) -> Result<()> {
    if signature.len() != 64 {
        return err!(Error::InvalidSignatureLength);
    }

  
    let recovered_pubkey = secp256k1_recover(message_hash, recovery_id, signature)
        .map_err(|_| Error::ErrorInRecovery)?;

        let recovered_pubkey_bytes = recovered_pubkey.to_bytes();

      let mut compressed = [0u8; 33];

      //check length
      match recovered_pubkey_bytes.len() {
          64 => {  
              compressed[0] = if recovery_id % 2 == 0 { 0x03 } else { 0x02 };
              compressed[1..].copy_from_slice(&recovered_pubkey_bytes[0..32]); // use the first 32 bytes
          },
          65 => {
                 // performing compression

              // Handle uncompressed public key throught prefixes
              compressed[0] = if recovered_pubkey_bytes[63] % 2 == 0 { 0x03 } else { 0x02 };
              compressed[1..].copy_from_slice(&recovered_pubkey_bytes[1..33]); // Use the first 32 bytes (1 to 32)
          },
          _ => {
              msg!("Unexpected recovered public key length: {}", recovered_pubkey_bytes.len());
              return err!(Error::InvalidPublicKeyLength);
          },
      }
  
      // Compare the compressed public key with the expected public key
      if compressed == expected_pubkey {
          msg!("Signature verification successful");
          Ok(())
        } else {
            msg!("Signature verification failed");
            msg!("compressed & expected {}, {}, {:?}, {:?},",compressed.len(),expected_pubkey.len(),compressed,expected_pubkey);
          err!(Error::SignatureVerificationFailed)
      }
  
}
