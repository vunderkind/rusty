
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
   account_info::{next_account_info, AccountInfo},
   entrypoint,
   entrypoint::ProgramResult,
   msg,
   program_error::ProgramError,
   pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
   pub name: String,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
   program_id: &Pubkey, // Public key of the account the GM program was loaded into
   accounts: &[AccountInfo], // The account to say GM to
   input: &[u8], // String input data, contains the name to say GM to
) -> ProgramResult {


 
   Ok(())
}