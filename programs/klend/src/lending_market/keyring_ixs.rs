use anchor_lang::prelude::{Account, Signer, Result};
use anchor_lang::{InstructionData, Key, ToAccountInfo, ToAccountMetas};
use keyring_network::common::types::EntityData;
use solana_program::instruction::Instruction;
use solana_program::program;
use solana_program::pubkey::Pubkey;

pub fn check_keyring_credentials<'info>(keyring_program: Pubkey, signer: &Signer<'info>, entity_mapping: &Account<'info, EntityData>, policy_id: u64) -> Result<()> {
    let check_credentials_accounts = keyring_network::accounts::CheckCredential {
        signer: signer.key(),
        entity_mapping: entity_mapping.key(),
    };

    let instruction = keyring_network::instruction::CheckCredential {
        policy_id,
        trading_address: signer.key(),
    };
    let data = instruction.data();
    let instruction = Instruction {
        program_id: keyring_program.clone(),
        accounts: check_credentials_accounts.to_account_metas(None),
        data,
    };

    program::invoke(
        &instruction,
        &[signer.to_account_info(), entity_mapping.to_account_info()],
    )?;
    Ok(())
}