use anchor_lang::prelude::*;
// use secp256k1_recover::secp256k1_recover;
// use sol_chainsaw::solana_sdk::{self, secp256k1_recover::secp256k1_recover};
// use solana_sdk::{entrypoint::ProgramResult, message, signature};
use solana_program::entrypoint::ProgramResult;
use solana_program::secp256k1_recover::secp256k1_recover;
declare_id!("7qEhDPXRiNuJZwSAeeqHj8uzJaRXgJ1k894J32K8dKvL");

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
        message_hash: Vec<u8>,
        recovery_id: u8,
        signature: Vec<u8>,
        expected_pubkey: Vec<u8>,
        nonce: u64,
    ) -> Result<()> {
        let accounts = ctx.accounts;
        //verify signature
        verify_signature(
            &message_hash,
            recovery_id,
            &signature,
            &expected_pubkey,
            nonce,
            &mut accounts.nonce_account,
        )?;

        let male_account = &accounts.male_account;
        msg!("get {} {}", male_account.name, male_account.age);
        Ok(())
    }
}

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

#[derive(Accounts)]
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

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
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
pub enum Error {
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
}

//helper function
pub fn verify_signature(
    message_hash: &[u8],    // hashed message (typically keccak256)
    recovery_id: u8,        // Recovery ID (0 or 1)
    signature: &[u8],       // Secp256k1 signature (64 bytes)
    expected_pubkey: &[u8], // Compressed public key (33 bytes)
    nonce: u64,
    nonce_account: &mut Nonce,
) -> Result<()> {
    if nonce <= nonce_account.last_nonce {
        return err!(Error::NonceAlreadyUsed);
    }
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
        }
        65 => {
            // performing compression

            // Handle uncompressed public key throught prefixes
            compressed[0] = if recovered_pubkey_bytes[63] % 2 == 0 {
                0x03
            } else {
                0x02
            };
            compressed[1..].copy_from_slice(&recovered_pubkey_bytes[1..33]); // Use the first 32 bytes (1 to 32)
        }
        _ => {
            msg!(
                "Unexpected recovered public key length: {}",
                recovered_pubkey_bytes.len()
            );
            return err!(Error::InvalidPublicKeyLength);
        }
    }

    // Compare the compressed public key with the expected public key
    if compressed == expected_pubkey {
        msg!("Signature verification successful");
        nonce_account.last_nonce = nonce;
        Ok(())
    } else {
        msg!("Signature verification failed");
        msg!(
            "compressed & expected {}, {}, {:?}, {:?},",
            compressed.len(),
            expected_pubkey.len(),
            compressed,
            expected_pubkey
        );
        err!(Error::SignatureVerificationFailed)
    }
}
