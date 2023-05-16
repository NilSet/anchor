use crate::state::game::*;
use anchor_lang::prelude::*;

pub fn setup_game(ctx: Context<SetupGame>, game_id: String, player_two: Pubkey) -> Result<()> {
    ctx.accounts
        .game
        .start([ctx.accounts.player_one.key(), player_two])
}

#[derive(Accounts)]
#[instruction(game_id: String)]
pub struct SetupGame<'info> {
    #[account(init, payer = player_one, space = Game::MAXIMUM_SIZE + 8, seeds = [game_id.as_bytes()], bump)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}
