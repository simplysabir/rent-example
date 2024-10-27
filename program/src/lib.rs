mod add;
mod initialize;

use add::*;
use initialize::*;
        
use rent_example_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&rent_example_api::ID, program_id, data)?;

    match ix {
        RentExampleInstruction::Initialize => process_initialize(accounts, data)?,
        RentExampleInstruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
