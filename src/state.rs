use borsh::{BorshDeserialize, BorshSerialize};
use solana_program:: {
    msg,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum AccountTag {
    Uninitialized,
    Mint,
    TokenAccount
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Mint {
    pub tag: AccountTag,
    pub authority: Pubkey,
    pub supply: u64,

}

impl Mint {

    pub fn load_unchecked(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        let mint = Self::try_from_slice(&account_info.data.borrow())?;
        return Ok(mint)
    }

    pub fn load_check(account_info: &AccountInfo) -> Result<Self, ProgramError>{
        let mint = Self::try_from_slice(&account_info.data.borrow())?;
        _ = mint.validate();
        Ok(mint)
    }

    pub fn validate(&self) -> ProgramResult{
        if self.tag != AccountTag::Mint {
            return  Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn save(&self, account_info: &AccountInfo) -> ProgramResult {
        let serialized_mint = self.serialize(&mut *account_info.data.borrow_mut())?;
        return Ok(serialized_mint);
    }

}


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenAccount {
    pub tag: AccountTag,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

impl TokenAccount {

    pub fn load_unchecked(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        Ok(Self::try_from_slice(&account_info.data.borrow())?)
    }
    pub fn validate(&self) -> ProgramResult{
        if self.tag != AccountTag::TokenAccount{
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn load_check(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        let account: TokenAccount = Self::try_from_slice(&account_info.data.borrow())?;
        _ = account.validate()?;
        return Ok(account)
    }

    pub fn save(&self, account_info: &AccountInfo) -> ProgramResult{
        let serialized_token_account = self.serialize(&mut *account_info.data.borrow_mut())?;
        return Ok(serialized_token_account);
    }

}


















