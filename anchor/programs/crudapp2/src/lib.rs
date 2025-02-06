#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod crudapp2 {
    use super::*;

  // 
  pub fn create_journal_entry(ctx: Context<CreateEntry>,) -> Result<()> {

  }

}

// start with program state
// state is where you hold all data
// since smart contracts, are stateless we store everything in program accounts
// therefore, all state is stored in program accounts

// need to create a custom createEntry struct
// all accounts passing through instruction handlers are wrapped in this "class"
#[derive(Accounts)]
pub struct CreateEntry<'info> {

  #[account(
    init,
    seeds = {title.as_bytes(), owner.key().as_ref()},
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