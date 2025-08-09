// program logic

use crate::instruction::EscrowInstruction;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::error::EscrowError;

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

        // Account 3: ???
        let escrow_account = next_account_info(account_info_iter)?;

        let rent = Rent::from_account_info(next_account_info(account_info_iter)?)?;
        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExampt.into())
        }

        Ok(())
    }
}
