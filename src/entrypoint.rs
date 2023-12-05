use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    entrypoint,
    entrypoint::{ProgramResult, ProcessInstruction}, 
    pubkey::Pubkey, 
    account_info::AccountInfo, program_error::ProgramError, nonce::state::Data
};

use crate::{state::Fundraiser, instruction::FundraiserInstruction, processor::Processor};


entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    instruction_data: &[u8]
) -> ProgramResult {

    let ix = FundraiserInstruction::unpack(instruction_data)?;

    match ix {
        FundraiserInstruction::Initialize(data) => {
            Processor::process_initialize(program_id, accounts, data)
        }
        FundraiserInstruction::Fund(data) => {
            Processor::process_fund(program_id, accounts, data)
        },
        FundraiserInstruction::CloseFundraiser => {
            Processor::process_close_fundraiser(program_id, accounts)
        },
    }
}

