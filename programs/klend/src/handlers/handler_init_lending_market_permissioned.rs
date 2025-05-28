use anchor_lang::prelude::*;

use crate::{
    handler_init_lending_market::InitLendingMarket, state::InitLendingMarketPermissionedParams,
};

pub fn process(
    ctx: Context<InitLendingMarket>,
    quote_currency: [u8; 32],
    policy_id: u64,
    keyring_program: Pubkey,
) -> Result<()> {
    let lending_market = &mut ctx.accounts.lending_market.load_init()?;

    lending_market.init_permissioned(InitLendingMarketPermissionedParams {
        quote_currency,
        lending_market_owner: ctx.accounts.lending_market_owner.key(),
        bump_seed: ctx.bumps.lending_market_authority,
        policy_id,
        keyring_program,
    });

    Ok(())
}
