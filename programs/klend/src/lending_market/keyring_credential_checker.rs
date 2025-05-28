use anchor_lang::prelude::{AccountDeserialize, AccountInfo, Result};
use anchor_lang::{require, Key};
use keyring_network::common::types::EntityData;
use solana_program::clock;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::LendingError;

pub fn check_keyring_credentials(
    policy_id: u64,
    keyring_program: Pubkey,
    signer: Pubkey,
    entity_mapping_account: &AccountInfo,
) -> Result<()> {
    let entity_mapping_data = entity_mapping_account.try_borrow_data()?;
    let entity_mapping = EntityData::try_deserialize(&mut entity_mapping_data.as_ref())?;
    let (actual_entity_mapping_key, _bump) = Pubkey::find_program_address(
        &[
            b"keyring_program".as_ref(),
            b"_entity_mapping".as_ref(),
            &policy_id.to_le_bytes(),
            &signer.to_bytes(),
        ],
        &keyring_program,
    );
    let current_timestamp = clock::Clock::get()?.unix_timestamp as u64;

    require!(
        actual_entity_mapping_key == entity_mapping_account.key(),
        LendingError::InvalidEntityMappingAccountPassed
    );
    require!(
        !entity_mapping.blacklisted && entity_mapping.exp > current_timestamp,
        LendingError::InvalidCredentials
    );

    Ok(())
}
