use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::{invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar, clock::Clock},
    hash::hash,
};

const SCORE_SIZE: usize = 8;
const HISTORY_SIZE: usize = 200;
const RECORD_SIZE: usize = 3; // player_move, program_move, result (0=loss,1=draw,2=win)
const DATA_SIZE: usize = SCORE_SIZE + (HISTORY_SIZE * RECORD_SIZE) + 1; // +1 byte for history_len
const SEED_PREFIX: &[u8] = b"score";

#[repr(C)]
pub struct GameData {
    pub score: u64,
    pub history_len: u8,
    pub history: [[u8; RECORD_SIZE]; HISTORY_SIZE],
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8], // player's choice: 0,1,2
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;
    let clock_sysvar = next_account_info(accounts_iter)?;

    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if instruction_data.len() != 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let player_choice = instruction_data[0];
    if player_choice > 2 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Derive PDA and check
    let seeds: &[&[u8]] = &[SEED_PREFIX, payer.key.as_ref()];
    let (expected_pda, bump) = Pubkey::find_program_address(seeds, program_id);
    if expected_pda != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    // Create PDA account if needed
    if pda_account.lamports() == 0 || pda_account.data_len() < DATA_SIZE {
        let rent = &Rent::from_account_info(rent_sysvar)?;
        let required_lamports = rent.minimum_balance(DATA_SIZE);

        let create_ix = system_instruction::create_account(
            payer.key,
            pda_account.key,
            required_lamports,
            DATA_SIZE as u64,
            program_id,
        );

        let signer_seeds: &[&[u8]] = &[SEED_PREFIX, payer.key.as_ref(), &[bump]];
        invoke_signed(
            &create_ix,
            &[payer.clone(), pda_account.clone(), system_program.clone()],
            &[signer_seeds],
        )?;

        let mut data = pda_account.try_borrow_mut_data()?;
        data[..SCORE_SIZE].copy_from_slice(&0u64.to_le_bytes());
        data[SCORE_SIZE] = 0; // history_len = 0
        for i in 0..HISTORY_SIZE * RECORD_SIZE {
            data[SCORE_SIZE + 1 + i] = 0;
        }
    }

    // Read & update game data
    let mut data = pda_account.try_borrow_mut_data()?;

    let mut score = u64::from_le_bytes(data[..SCORE_SIZE].try_into().unwrap());
    let mut history_len = data[SCORE_SIZE];

    // Generate random choice (0..2) from hash(slot + payer)
    let clock = Clock::from_account_info(clock_sysvar)?;
    let mut seed_data = Vec::new();
    seed_data.extend_from_slice(&clock.slot.to_le_bytes());
    seed_data.extend_from_slice(payer.key.as_ref());

    let hash_result = hash(&seed_data);
    let program_choice = (hash_result.as_ref()[0] % 3) as u8;

    // Determine result: 0=loss, 1=draw, 2=win for player
    // win if (player_choice - program_choice + 3) % 3 == 1
    // draw if equal
    // loss otherwise
    let result = if player_choice == program_choice {
        1u8
    } else if (player_choice + 3 - program_choice) % 3 == 1 {
        2u8
    } else {
        0u8
    };

    // Update score: +2 win, +1 draw, +0 loss
    score = score.checked_add(match result {
        2 => 2,
        1 => 1,
        _ => 0,
    }).ok_or(ProgramError::Custom(0))?;

    // Update data score
    data[..SCORE_SIZE].copy_from_slice(&score.to_le_bytes());

    // Update history
    // Shift history left if full
   if history_len as usize == HISTORY_SIZE {
    for i in 0..(HISTORY_SIZE - 1) {
        let src_start = SCORE_SIZE + 1 + (i + 1) * RECORD_SIZE;
        let dest_start = SCORE_SIZE + 1 + i * RECORD_SIZE;

        let temp = data[src_start..src_start + RECORD_SIZE].to_vec();
        data[dest_start..dest_start + RECORD_SIZE].copy_from_slice(&temp);
    }
    history_len = HISTORY_SIZE as u8;
} else {
    history_len += 1;
    data[SCORE_SIZE] = history_len;
}

    // Add new record at end
    let new_record_pos = SCORE_SIZE + 1 + (history_len as usize -1) * RECORD_SIZE;
    data[new_record_pos] = player_choice;
    data[new_record_pos + 1] = program_choice;
    data[new_record_pos + 2] = result;

    Ok(())
}
