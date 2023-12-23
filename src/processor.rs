use solana_program::{entrypoint::ProgramResult, account_info::{AccountInfo, next_account_info}, pubkey::Pubkey, system_program, program_error::ProgramError, system_instruction, program::{invoke, invoke_signed}, rent::Rent, sysvar::Sysvar, msg};

use crate::{instruction::{FundraiserInstruction, InitializeInstructionData, FundInstruction}, state::Fundraiser};

pub struct Processor;

impl Processor {
    pub fn process_initialize(
        program_id: &Pubkey, 
        accounts: &[AccountInfo], 
        instruction: InitializeInstructionData
    ) -> ProgramResult {

        let iter = &mut accounts.iter();

        // program derived address
        let initializer_account = next_account_info(iter)?;
        let balance_pda_account = next_account_info(iter)?;
        let fundraser_state_account = next_account_info(iter)?;
        let system_program = next_account_info(iter)?;


        // rent exempt
        let rent = Rent::get()?;
        let minimum_balance = rent.minimum_balance(73);

        let (state_account_pda, bump) = Pubkey::find_program_address(
            &[
                b"fundraiser", 
                initializer_account.key.as_ref(), 
                &[instruction.extra_seed]
            ],
            program_id
        );

        if state_account_pda != *fundraser_state_account.key {
            return Err(ProgramError::Custom(1));
        }

        let create_ix = system_instruction::create_account(
            initializer_account.key, 
            fundraser_state_account.key, 
            minimum_balance, 
            73, 
            program_id
        );

        invoke_signed(&create_ix, &[
            initializer_account.clone(),
            fundraser_state_account.clone(),
        ], &[
            &[
                b"fundraiser", 
                initializer_account.key.as_ref(),
                &[instruction.extra_seed],
                &[bump]
            ]
        ])?;

        let mut fundraiser_state = Fundraiser::unpack_from_account(fundraser_state_account)?;
        // if fundraiser_state.is_initialized {
        //     return Err(ProgramError::AccountAlreadyInitialized);
        // }

        let (balance_calculated_pda, bump) = Pubkey::find_program_address(
            &[
                b"balance", 
                fundraser_state_account.key.as_ref()
            ], 
            program_id
        );

        if balance_calculated_pda != *balance_pda_account.key {
            return Err(ProgramError::Custom(2));
        }

        fundraiser_state.is_initialized = true;
        fundraiser_state.balance_pda_account = *balance_pda_account.key;
        fundraiser_state.goal_amount = instruction.goal_amount;
        fundraiser_state.initializer_pubkey = *initializer_account.key;

        fundraiser_state.pack_to_account(fundraser_state_account);

        Ok(())
    }

    pub fn process_fund(
        program_id: &Pubkey, 
        accounts: &[AccountInfo<'_>], 
        instruction: FundInstruction) -> ProgramResult {

        let iter = &mut accounts.iter();


        

        let funder_account = next_account_info(iter)?;
        let funding_pda = next_account_info(iter)?;
        let system_program = next_account_info(iter)?;

        let ix = system_instruction::transfer(
                funder_account.key,
                 funding_pda.key, 
                 instruction.funding_amount);
        invoke(&ix, &[
            funder_account.clone(),
            funding_pda.clone(),
            system_program.clone()
        ])?;
        
        Ok(())
    }

    pub fn process_close_fundraiser(program_id: &Pubkey, accounts: &[AccountInfo<'_>]) -> ProgramResult {

        let iter = &mut accounts.iter();
        let initializer_account = next_account_info(iter)?;
        let balance_pda = next_account_info(iter)?;
        let fundraiser_state_account = next_account_info(iter)?;
        let system_program = next_account_info(iter)?;


        let state = Fundraiser::unpack_from_account(fundraiser_state_account)?;

        
        
        if state.goal_amount > balance_pda.lamports() {
            return Err(ProgramError::Custom(1));
        }

        let ix = system_instruction::transfer(
            balance_pda.key, 
            initializer_account.key, 
            balance_pda.lamports()
        );

        let (balance_calculated_pda, bump) = Pubkey::find_program_address(
            &[
                b"balance", 
                fundraiser_state_account.key.as_ref()
            ], 
            program_id
        );

        invoke_signed(
            &ix, 
            &[
                balance_pda.clone(),
                initializer_account.clone()
            ], 
            &[
                &[
                    b"balance",
                    fundraiser_state_account.key.as_ref(),
                    &[bump]
                ]
            ]
        )?;

        // transfer to initializer account
        **initializer_account.lamports.borrow_mut() = initializer_account
            .lamports()
            .checked_add(fundraiser_state_account.lamports())
            .ok_or(ProgramError::InsufficientFunds)?;

        **fundraiser_state_account.lamports.borrow_mut() = 0;
        


        Ok(())
    }
}