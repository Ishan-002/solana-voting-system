use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("9tzc763yy5v74LKwwjRdUwJyB1mQMxcrgVntpehcnm5H");

#[program]
pub mod solana_voting_system {
    use super::*;


    pub fn send_vote(ctx: Context<SendVote>) -> Result<()> {
        let chosen = ctx.accounts.option.option;
        let mut updatectx: Context<AccessCount>;

        update_count(updatectx, chosen);

        Ok(())
    }

    pub fn update_count(ctx: Context<AccessCount>, option: usize) -> Result<()>{
        ctx.accounts.full_count.vote_vec[option] =  ctx.accounts.full_count.vote_vec[option] + 1;
        Ok(()) 
    }

    pub fn add_option(ctx: Context<AddOption>, input: &String) -> Result<()>{
        let mut addctx: Context<AccessCount>;

        addctx.accounts.full_count.option_vec.push(*input);
        addctx.accounts.full_count.vote_vec.push(0);
        addctx.accounts.full_count.count = addctx.accounts.full_count.count + 1;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct SendVote<'info> {
    #[account(init, payer = author, space=100)]
    pub option: Account<'info, Option>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct AddOption<'info> {
    pub new_candidate: Account<'info, Candidate>
}

#[derive(Accounts)]
pub struct AccessCount<'info>{
    pub full_count: Account<'info, AllOptionsVotes>

}

#[account]
pub struct AllOptionsVotes{
    pub option_vec: Vec<String>,
    pub vote_vec: Vec<i64>,
    pub count: u64
}

#[account]
pub struct Option{
    pub option: usize
}

#[account]
pub struct Candidate{
    pub candidate: String
}