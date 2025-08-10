// program objects, (de)serializing state

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

pub struct Escrow {
    // tracks whether an escrow account is in use
    pub is_initialized: bool,
    // Alice public key
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
        self.is_initialized
    }
}

impl Pack for Escrow {
    const LEN: usize = 105;
    /// serializing state from byte-sequence to Rust object
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
        ) = array_refs![src, 1, 32, 32, 32, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Escrow {
            is_initialized,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(
                *initializer_token_to_receive_account_pubkey,
            ),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })
    }

    /// deserializing state from Rust object to byte-sequence
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Escrow::LEN];

        let (
            is_initialized_dst,
            initializer_pubkey_dst,
            temp_token_account_pubkey_dst,
            initializer_token_to_receive_account_pubkey_dst,
            expected_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8];

        let Escrow {
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        initializer_token_to_receive_account_pubkey_dst
            .copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
        *expected_amount_dst = expected_amount.to_le_bytes();
    }

    // Alternative
    // fn pack_into_slic(&self, dst: &mut [u8]) {
    //     // Safety check: Make sure the buffer is big enough
    //     assert!(dst.len() >= Escrow::LEN, "dst too small");

    //     // 0..1  → is_initialized (bool as u8)
    //     dst[0] = self.is_initialized as u8;

    //     // 1..33 → initializer_pubkey ([u8; 32])
    //     dst[1..33].copy_from_slice(self.initializer_pubkey.as_ref());

    //     // 33..65 → temp_token_account_pubkey ([u8; 32])
    //     dst[33..65].copy_from_slice(self.temp_token_account_pubkey.as_ref());

    //     // 65..97 → initializer_token_to_receive_account_pubkey ([u8; 32])
    //     dst[65..97].copy_from_slice(
    //         self.initializer_token_to_receive_account_pubkey.as_ref(),
    //     );

    //     // 97..105 → expected_amount (u64 → [u8; 8] in little-endian)
    //     dst[97..105].copy_from_slice(&self.expected_amount.to_le_bytes());
    // }
}
