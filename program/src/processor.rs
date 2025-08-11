// program logic

use crate::instruction::EscrowInstruction;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token;

use crate::{error::EscrowError, state::Escrow};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instrucion: InitEscrow");
                Self::process_init_escrow(accounts, amount, program_id)
            }
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        // Account 0: Alice's main account
        let initializer = next_account_info(account_info_iter)?;

        // Check if Alice is a signer to her account
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Account 1: Temporary account where Alice will send token X
        let temp_token_account = next_account_info(account_info_iter)?;

        // There is no need to check if the `temp_token_account` is owned by the Token program
        // because later on when we request that the Token program transfer ownership of the
        // `temp_token_account`, it will fail if it doesn't own it...so no need for the check.

        // Account 2: Token Alice expects to receive
        let token_to_receive_account = next_account_info(account_info_iter)?;

        // ...from Solana’s SPL Token program crate, and it’s basically a convenience function
        // that tells you “what is the public key of the official SPL Token program on Solana?”
        // // Checking if the token account is owned by the official Token program
        if *token_to_receive_account.owner == spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        // Account 3: where does the escrow account come from???
        let escrow_account = next_account_info(account_info_iter)?;

        let rent = Rent::from_account_info(next_account_info(account_info_iter)?)?;
        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExampt.into());
        }

        // unpack escrow information from slice
        let mut escrow_info: Escrow = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;

        escrow_info.is_initialized = true;
        escrow_info.initializer_pubkey = *initializer.key;
        escrow_info.temp_token_account_pubkey = *temp_token_account.key;
        escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        escrow_info.expected_amount = amount;

        Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;

        // generate a Program Derived Address with a seed that will be used when trying to sign
        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

        // Account 4: The token program
        let token_program = next_account_info(account_info_iter)?;

        // Create the transaction to transfer token account ownership
        // Signature Extension: Be careful what you account you forward to CPI as signers
        let owner_change_ix = spl_token::instruction::set_authority(
            // token program ID
            token_program.key,
            // account to be transferred from Alice to Escrow
            temp_token_account.key,
            // Account to be new authority Public Derived Address of the Escrow
            Some(&pda),
            // Instruction type
            spl_token::instruction::AuthorityType::AccountOwner,
            // Account owner's public key
            initializer.key,
            // signer public key
            &[&initializer.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");

        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }
}
