use cartesi_solana::anchor_lang::solana_program::entrypoint::ProgramResult;
use cartesi_solana::executor::create_executor;

use mpl_token_metadata::{error::MetadataError, processor};
use solana_program::{account_info::AccountInfo, program_error::PrintProgramError, pubkey::Pubkey};
pub(crate) use std::io;

fn main() -> io::Result<()> {
    let mut executor = create_executor();
    executor.get_processor_args(|program_id, accounts, data| {
        process_instruction(&program_id, accounts, &data).unwrap();
    });
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
