#![no_std]
#![allow(clippy::type_complexity)]

mod call_async;
pub mod call_sync;
mod call_transf_exec;
mod contract_change_owner;
mod contract_deploy;
mod contract_upgrade;
mod dct;
mod nft;
mod roles;
mod sft;
mod storage;

dharitri_wasm::imports!();

/// Test contract for investigating contract calls.
#[dharitri_wasm::contract]
pub trait Forwarder:
    call_sync::ForwarderSyncCallModule
    + call_async::ForwarderAsyncCallModule
    + call_transf_exec::ForwarderTransferExecuteModule
    + contract_change_owner::ChangeOwnerModule
    + contract_deploy::DeployContractModule
    + contract_upgrade::UpgradeContractModule
    + dct::ForwarderDctModule
    + sft::ForwarderSftModule
    + nft::ForwarderNftModule
    + roles::ForwarderRolesModule
    + storage::ForwarderStorageModule
{
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn send_moax(
        &self,
        to: &ManagedAddress,
        amount: &BigUint,
        #[var_args] opt_data: OptionalArg<ManagedBuffer>,
    ) {
        let data = match opt_data {
            OptionalArg::Some(data) => data,
            OptionalArg::None => ManagedBuffer::new(),
        };
        self.send().direct_moax(to, amount, data);
    }
}
