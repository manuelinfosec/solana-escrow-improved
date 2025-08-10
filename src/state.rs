// program objects, (de)serializing state

use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

pub struct Escrow {
    pub is_initiailized: bool,
    pub initializer_pubkey: Pubkey,
    // this is where Bob receives X tokens from
    pub temp_token_account_pubkey: Pubkey,
    // this is where Bob's Y tokens go
    pub initializer_token_to_receive_account_pubkey: Pubkey,
    // confirm that Bob sends enough of his tokens
    pub expected_amount: u64,
}

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initiailized
    }
}
