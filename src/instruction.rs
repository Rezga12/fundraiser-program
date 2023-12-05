// data allocator

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::program_error::ProgramError;


pub enum FundraiserInstruction {
    
    // [signer] initializer
    // [writeable] fund pda account
    // [writeable] fundraiser metadata account
    // [] system program account
    Initialize (InitializeInstructionData),

    // [signer] funder
    // [writeable] fund pda account
    // [] system program account
    Fund (FundInstruction),

    CloseFundraiser,

}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct InitializeInstructionData {
    pub goal_amount: u64,
    pub extra_seed: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct FundInstruction {
    pub funding_amount: u64
}



impl FundraiserInstruction {
    pub fn unpack(data: &[u8]) -> Result<FundraiserInstruction, ProgramError> {
        let (ix_byte, instruction_data) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match ix_byte {
            0 => {
                let ix = InitializeInstructionData::try_from_slice(instruction_data)?;
                return Ok(FundraiserInstruction::Initialize(ix));
            },
            1 => {
                let ix = FundInstruction::try_from_slice(instruction_data)?;
                return Ok(FundraiserInstruction::Fund(ix))
            },
            2 => {
                return Ok(FundraiserInstruction::CloseFundraiser)
            }
            _ => Err(ProgramError::InvalidInstructionData)
        }

    }
}