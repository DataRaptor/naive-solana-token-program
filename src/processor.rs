use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    msg,
    pubkey::Pubkey,
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    entrypoint::ProgramResult};


use crate::instruction::TokenInstruction;
use crate::state::{Mint, AccountTag, TokenAccount};


pub fn assert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}


pub struct Processor{}

impl Processor {

    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult {

        let instruction: TokenInstruction = TokenInstruction::try_from_slice(
            instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;
        
        let accounts_iter = &mut accounts.iter();

        match instruction {

            TokenInstruction::InitializeMint => {
                msg!("Initializing Mint");
                let mint_account_info: &AccountInfo = next_account_info(accounts_iter)?;
                let mint_authority : &AccountInfo = next_account_info(accounts_iter)?;
                _ = assert_with_msg(
                    mint_authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Mint Authority must sign",
                );
                let mut mint: Mint = Mint::load_unchecked(mint_account_info)?;
                mint.tag = AccountTag::Mint;
                mint.authority = *mint_authority.key;
                mint.supply = 0;
                _ = mint.save(mint_account_info);
            }
            TokenInstruction::InitializeTokenAccount => {
                msg!("Initializing Token Account");
                let token_account_account_info: &AccountInfo = next_account_info(accounts_iter)?;
                let mint_account_info: &AccountInfo = next_account_info(accounts_iter)?;
                let mut token_account: TokenAccount = TokenAccount::load_unchecked(& token_account_account_info)?;
                token_account.tag = AccountTag::TokenAccount;
                token_account.owner = *token_account_account_info.key;
                token_account.mint = *mint_account_info.key;
                token_account.amount = 0;
                _ = token_account.save(token_account_account_info);
            }
            TokenInstruction::Mint { amount } => {
                msg!("Minting {:?} Tokens", amount);
                let token_account_account_info: &AccountInfo = next_account_info(accounts_iter)?;
                let mint_account_info: &AccountInfo = next_account_info(accounts_iter)?;
                let mint_authority: &AccountInfo = next_account_info(accounts_iter)?;
                let mut token_account: TokenAccount = TokenAccount::load_check(& token_account_account_info)?;
                let mut mint: Mint = Mint::load_check(mint_account_info)?;
                _ = assert_with_msg(
                    mint_authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Mint Authority Must be a signer"
                );
                _ = assert_with_msg(
                    mint.authority == *mint_authority.key,
                    ProgramError::MissingRequiredSignature,
                    "Mint Authority Mismatch - you cant mint more tokens with this signature"
                );
                mint.supply += amount;
                token_account.amount += amount;
                token_account.save(token_account_account_info)?;
                mint.save(mint_account_info)?;
            }
            TokenInstruction::Burn { amount } => {
                msg!("Burning {:?} Tokens", amount);
                let token_account_account_info: &AccountInfo = next_account_info(accounts_iter)?;
                let mint_account_info: &AccountInfo = next_account_info(accounts_iter)?;
                let mint_authority: &AccountInfo = next_account_info(accounts_iter)?;
                let mut token_account: TokenAccount = TokenAccount::load_check(&token_account_account_info)?;
                let mut mint: Mint = Mint::load_check(&mint_account_info)?;
                _ = assert_with_msg(
                    mint_authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Mint Authority Must be a signer"
                );
                _ = assert_with_msg(
                    mint.authority == *mint_authority.key,
                    ProgramError::MissingRequiredSignature,
                    "Mint Authority Mismatch - you cant mint more tokens with this signature"
                );
                // This does not look safe.
                mint.supply -= amount;
                token_account.amount -= amount;
                token_account.save(token_account_account_info)?;
                mint.save(mint_account_info)?;
            }
            TokenInstruction::Transfer { amount } => {
                // It would be nice here if we didn't assume we already have the token accounts.
                // Oh well...
                msg!("Transfering {:?} Tokens", amount);
                let source_token_account_account_info: &AccountInfo = next_account_info(accounts_iter)?;
                let destination_token_account_account_info: &AccountInfo = next_account_info(accounts_iter)?;
                let owner: &AccountInfo = next_account_info(accounts_iter)?;
                let mut source_token_account: TokenAccount = TokenAccount::load_check(& source_token_account_account_info)?;
                let mut destination_token_account: TokenAccount = TokenAccount::load_check(& destination_token_account_account_info)?;
                assert_with_msg(
                    owner.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Token owner must sign",
                )?;
                assert_with_msg(
                    source_token_account.owner == *owner.key,
                    ProgramError::MissingRequiredSignature,
                    "Token owner mismatch",
                )?;
                assert_with_msg(
                        source_token_account.amount >= amount, 
                    ProgramError::InvalidAccountData,
                    "Attempting to transfer more than account balance",
                )?;
                assert_with_msg(
                    source_token_account.mint == destination_token_account.mint, 
                    ProgramError::InvalidAccountData,
                    "Token account mints do not match",
                )?;

                source_token_account.amount -= amount;
                destination_token_account.amount += amount;
                source_token_account.save(& source_token_account_account_info)?;
                destination_token_account.save(& destination_token_account_account_info)?;
            }
        }
        Ok(())
    }
}

