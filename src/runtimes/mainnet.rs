#[subxt::subxt(runtime_metadata_path = "artifacts/mainnet.scale")]
pub mod mainnet {
    #[subxt(substitute_type = "frame_support::storage::bounded_vec::BoundedVec")]
    use ::sp_std::vec::Vec;
}
use super::types;
pub use mainnet::runtime_types::frame_system::AccountInfo;
pub use mainnet::runtime_types::pallet_balances::AccountData;
pub use mainnet::runtime_types::pallet_smart_contract::types::Contract;
pub use mainnet::runtime_types::pallet_tfgrid::{
    farm::FarmName,
    interface::{InterfaceIp, InterfaceMac, InterfaceName},
    pub_config::{Domain, GW4, GW6, IP4, IP6},
    pub_ip::{GatewayIP, PublicIP},
    twin::TwinIp,
    types::Twin as TwinData,
};
pub use mainnet::runtime_types::tfchain_support::types::{
    Farm as FarmData, Interface, Node as NodeData, PublicConfig, PublicIP as PublicIpData, IP,
};
use subxt::ext::{sp_core::H256, sp_runtime::AccountId32};

use subxt::Error;

pub type Twin = TwinData<TwinIp, AccountId32>;

pub type PublicIpOf = PublicIpData<PublicIP, GatewayIP>;
pub type Farm = FarmData<FarmName, PublicIpOf>;

pub type IPv4 = IP<IP4, GW4>;
pub type IPv6 = IP<IP6, GW6>;
pub type PublicConfigOf = PublicConfig<IPv4, Option<IPv6>, Option<Domain>>;
pub type InterfaceOf = Interface<InterfaceName, InterfaceMac, Vec<InterfaceIp>>;
pub type Node = NodeData<PublicConfigOf, InterfaceOf>;

use crate::client::{Client, KeyPair};

pub use mainnet::tft_bridge_module::events::BurnTransactionReady;
pub use mainnet::tft_bridge_module::events::BurnTransactionSignatureAdded;
pub use mainnet::tft_bridge_module::events::MintTransactionProposed;

pub type SystemAccountInfo = AccountInfo<u32, AccountData<u128>>;

pub async fn create_twin(
    cl: &Client,
    kp: &KeyPair,
    ip: Option<String>,
    _pk: Option<String>,
) -> Result<u32, Error> {
    let create_twin_tx = mainnet::tx()
        .tfgrid_module()
        .create_twin(ip.unwrap().as_bytes().to_vec());

    let signer = kp.signer();

    let create_twin = cl
        .api
        .tx()
        .sign_and_submit_then_watch_default(&create_twin_tx, signer.as_ref())
        .await?
        .wait_for_finalized_success()
        .await?;

    let twin_create_event =
        create_twin.find_first::<mainnet::tfgrid_module::events::TwinStored>()?;

    if let Some(event) = twin_create_event {
        Ok(event.0.id)
    } else {
        Err(Error::Other(String::from("failed to create twin")))
    }
}

pub async fn update_twin(
    cl: &Client,
    kp: &KeyPair,
    ip: Option<String>,
    _pk: Option<String>,
) -> Result<H256, Error> {
    let update_twin_tx = mainnet::tx()
        .tfgrid_module()
        .update_twin(ip.unwrap().as_bytes().to_vec());

    let signer = kp.signer();

    let update_twin = cl
        .api
        .tx()
        .sign_and_submit_then_watch_default(&update_twin_tx, signer.as_ref())
        .await?
        .wait_for_finalized_success()
        .await?;

    let twin_create_event =
        update_twin.find_first::<mainnet::tfgrid_module::events::TwinUpdated>()?;

    if let Some(_) = twin_create_event {
        Ok(update_twin.block_hash())
    } else {
        Err(Error::Other(String::from("failed to create twin")))
    }
}

pub async fn sign_terms_and_conditions(
    cl: &Client,
    kp: &KeyPair,
    document_link: String,
    document_hash: String,
) -> Result<H256, Error> {
    let sign_tandc_tx = mainnet::tx().tfgrid_module().user_accept_tc(
        document_link.as_bytes().to_vec(),
        document_hash.as_bytes().to_vec(),
    );

    let signer = kp.signer();

    let sign_tandc = cl
        .api
        .tx()
        .sign_and_submit_then_watch_default(&sign_tandc_tx, signer.as_ref())
        .await?
        .wait_for_finalized_success()
        .await?;

    Ok(sign_tandc.block_hash())
}

pub async fn get_twin_by_id(
    cl: &Client,
    id: u32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::Twin>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&mainnet::storage().tfgrid_module().twins(id), at_block)
        .await?
        .map(types::Twin::from))
}

pub async fn get_twin_id_by_account(
    cl: &Client,
    account: AccountId32,
    at_block: Option<types::Hash>,
) -> Result<Option<u32>, Error> {
    cl.api
        .storage()
        .fetch(
            &mainnet::storage()
                .tfgrid_module()
                .twin_id_by_account_id(account),
            at_block,
        )
        .await
}

pub async fn get_contract_by_id(
    cl: &Client,
    id: u64,
    at_block: Option<types::Hash>,
) -> Result<Option<types::Contract>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(
            &mainnet::storage().smart_contract_module().contracts(id),
            at_block,
        )
        .await?
        .map(types::Contract::from))
}

pub async fn get_node_by_id(
    cl: &Client,
    id: u32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::TfgridNode>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&mainnet::storage().tfgrid_module().nodes(id), at_block)
        .await?
        .map(types::TfgridNode::from))
}

pub async fn get_farm_by_id(
    cl: &Client,
    id: u32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::TfgridFarm>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&mainnet::storage().tfgrid_module().farms(id), at_block)
        .await?
        .map(types::TfgridFarm::from))
}

pub async fn get_block_hash(
    cl: &Client,
    block_number: Option<types::BlockNumber>,
) -> Result<Option<types::Hash>, Error> {
    cl.api.rpc().block_hash(block_number).await
}

pub async fn get_balance(
    cl: &Client,
    account: &AccountId32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::SystemAccountInfo>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&mainnet::storage().system().account(account), at_block)
        .await?
        .map(|t| types::SystemAccountInfo::from(t)))
}
