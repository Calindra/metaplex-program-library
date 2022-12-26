pub(crate) use std::io;

use cartesi_solana::{adapter::{get_processor_args, persist_accounts}, cartesi_stub::CartesiStubs};
use mpl_token_metadata::{processor, error::MetadataError};
use solana_program::{pubkey::Pubkey, account_info::AccountInfo, program_error::PrintProgramError};
use cartesi_solana::anchor_lang::solana_program::entrypoint::ProgramResult;

fn main() -> io::Result<()> {
    let (program_id, accounts, data, delete) = get_processor_args();
    
    solana_program::program_stubs::set_syscall_stubs(Box::new(CartesiStubs { program_id: program_id.clone() }));

    process_instruction(&program_id, &accounts, &data).unwrap();
    persist_accounts(&accounts, delete);
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