pub(crate) use std::io;

use cartesi_solana::adapter::{get_processor_args, persist_accounts};
use mpl_token_metadata::{processor, error::MetadataError};
use solana_program::{pubkey::Pubkey, account_info::AccountInfo, program_error::PrintProgramError};
use cartesi_solana::anchor_lang::solana_program::entrypoint::ProgramResult;

fn main() -> io::Result<()> {
    let (program_id, accounts, data) = get_processor_args();
    process_instruction(&program_id, &accounts, &data).unwrap();
    persist_accounts(&accounts);
    Ok(())
}

fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<MetadataError>();
        return Err(error);
    }
    Ok(())
}