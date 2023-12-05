use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{pubkey::Pubkey, program_error::ProgramError, account_info::AccountInfo, entrypoint::ProgramResult};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct Fundraiser {
    pub initializer_pubkey: Pubkey, // 32
    pub balance_pda_account: Pubkey, // 32
    pub goal_amount: u64, // 8
    pub is_initialized: bool, // 1 byte
}

impl Fundraiser {
    pub const DATA_LENGTH: u64 = 73;
    pub fn unpack_from_account(account: &AccountInfo) -> Result<Fundraiser, ProgramError>{
        
        let data = account.try_borrow_data()?;
        let bytes = data.as_ref();
        let result = Fundraiser::try_from_slice(bytes)?;

        return Ok(result);
    }

    pub fn pack_to_account(self, account: &AccountInfo) -> ProgramResult{

        let bytes = self.try_to_vec()?;
        let mut data = account.data.borrow_mut();
        data.copy_from_slice(&bytes[..]);

        Ok(())

    }
}