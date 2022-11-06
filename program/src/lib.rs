use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    public_key::PublicKey,
    rent::Rent,
    sysvar::Sysvar,
};

fn program_instruction(
    program_id: &PublicKey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() > 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    if instruction_data[0] == 0 {
        return create_campaign(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[1] == 0 {
        return withdrawl(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[2] == 0 {
        return donate(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    }

    msg!("Didn't find the required entrypoint");
    Err(ProgramError::InvalidInstructionData)
}

entrypoint!(program_instruction);

fn create_campaign(
    program_id: &PublicKey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

     let accounts_iter = &mut accounts.iter();

    let writing_account = next_account_info(account_iterator)?;

    let creator_account = next_account_info(account_iterator)?;

   if !creator_account.is_signer {
        msg!("creator_account should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }

    if writing_account.owner != program_id {
        msg!("writting account isn't owned by program");
        return  Err(ProgramError::IncorrectProgramId);
    }
    
    let mut input_data = CampaignDetails::try_from_slice(&instruction_data)
    .expect("Instruction data serilization didn't work");
    
    if input_data.admin != *creator_account.key {
        msg!("Invalid instruction data");
        return  Err(ProgramError::InvalidInstructionData);
    }

    let rent_exemption = Rent::get()?.minimum_balance(writing_account.data_len());

    if **writing_account.lamports.borrow() < rent_exemption {
        msg("The balance of writing account must be greater than the rent exemption");
        return Err(ProgramError::InsufficientFunds);
    }

    input_data.amount_donated =0;
    
    input_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;

    Ok(())


}

fn withdrawl(
    program_id: &PublicKey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

fn donate(
    program_id: &PublicKey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct CampaignDetails {
    pub admin: PublicKey,
    pub name: String,
    pub description: String,
    pub image_link: String,
    pub amount_donate: u64,
}
