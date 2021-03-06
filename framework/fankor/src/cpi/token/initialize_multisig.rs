use crate::errors::Error;
use crate::prelude::FankorResult;
use solana_program::account_info::AccountInfo;

pub struct CpiInitializeMultisig<'info> {
    pub multisignature: AccountInfo<'info>,
    pub signers: Vec<AccountInfo<'info>>,
}

pub fn initialize_multisig(
    accounts: CpiInitializeMultisig,
    m: u8,
    signer_seeds: &[&[&[u8]]],
) -> FankorResult<()> {
    let signer_pubkeys = accounts.signers.iter().map(|v| v.key).collect::<Vec<_>>();
    let ix = spl_token::instruction::initialize_multisig(
        &spl_token::ID,
        accounts.multisignature.key,
        &signer_pubkeys,
        m,
    )?;

    let mut infos = Vec::with_capacity(1 + accounts.signers.len());
    infos.push(accounts.multisignature);
    infos.extend(accounts.signers.into_iter());

    solana_program::program::invoke_signed(&ix, &infos, signer_seeds)
        .map_or_else(|e| Err(Error::ProgramError(e)), |_| Ok(()))
}
