#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program] 
pub mod crudapp2 {
    use super::*;

  // "C"reate for crud
  // the title is being passed throughto instruction, but not the actual create entry struct
  // fix this by using instruction macro to pass this param to the struct
  pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
    let journal_entry: &mut Account<JournalEntryState> = &mut ctx.accounts.journal_entry;
    journal_entry.owner = *ctx.accounts.owner.key;
    journal_entry.title = title;
    journal_entry.message = message;
    Ok(())
  }

  // "R"ead is just reading blockchain entry

  // "U"pdate the journal entry on chain
  // why is the title commented out here? this is because the title isnt being used in the handler, just the struct
  pub fn update_journal_entry(ctx: Context<UpdateEntry>, _title: String, message: String) -> Result <()> {

    let journal_entry = &mut ctx.accounts.journal_entry; // if not using acc context, 
    journal_entry.message = message;

    Ok(())

  }

  // "D"elete the journal entry on chain
  // in rust, if a param isnt used in a function, but it being passed to another struct, best prac is to use _ before the var name 
  pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, _title: String) -> Result<()> {
  
    // no logic here, since deleting on chain takes place in the data structure itself

    Ok(())

  }



}

// start with program state
// state is where you hold all data
// since smart contracts, are stateless we store everything in program accounts
// therefore, all state is stored in program accounts

// need to create a custom createEntry struct
// all accounts passing through instruction handlers are wrapped in this "class"
#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {

  #[account(
    init,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    space = 8 + JournalEntryState::INIT_SPACE,
    payer = owner,
  )]

  // naming the account specfied as above
  pub journal_entry: Account<'info, JournalEntryState>,

  // owner account
  // since the owner will be paying, the state of the account will change, hence it must
  // be verbosely stated that this is mutable
  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
#[instruction(title: String)] // title is being passed through to instruction
pub struct UpdateEntry<'info> {

  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()], // the title is used here as a seed to calculate pda
    bump,
    realloc = 8 + JournalEntryState::INIT_SPACE, // specfic constraint, where it calculates new space
    realloc::payer = owner, // this is the space and rent allocations being redistributed
    realloc::zero = true, // setting original calc to 0 before realloc
  )]

  pub journal_entry: Account<'info, JournalEntryState>,

  #[account(mut)]
  pub owner: Signer<'info>,

  // system program
  pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {

  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    close = owner, // custom constraint, if this instruction is run, it closes the account
    // only will work if the pubkey of the account is passed as the owner
  )]  

  pub journal_entry: Account<'info, JournalEntryState>,

  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,

}

// everything for journal
#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {

  pub owner: Pubkey,
  #[max_len(50)]
  pub title: String,
  #[max_len(1000)]
  pub message: String,

}