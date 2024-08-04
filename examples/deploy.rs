use anybuf::Anybuf;
use cosmos_sdk_proto::Any;
use cosmwasm_std::Uint128;
use cw_orch::{
    anyhow::{self, Ok},
    daemon::{networks, TxSender},
    prelude::*,
};
use cw_post::{
    models::{Config, FeeParams},
    msg::{InstantiateMsg, MigrateMsg, NodeInitArgs},
    tokens::Token,
    CwPostContract, CwPostExecuteMsgFns, CwPostQueryMsgFns,
};
use dotenv;
use pretty_env_logger;

const FEE_COLLECTION_ADDR: &str = "juno1rec44j9xq8aj4w5kun796f89njzvdlezwk7cy4";

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let network = networks::UNI_6;
    let chain = DaemonBuilder::new(network.clone()).build()?;

    let cw_post_contract = CwPostContract::new(chain.clone());
    let sender = chain.sender().address();

    cw_post_contract.upload_if_needed()?;

    if cw_post_contract.address().is_err() {
        cw_post_contract.instantiate(
            &InstantiateMsg {
                config: Config {
                    token: Token::Denom("ujunox".to_string()),
                    fee_recipient: Some(Addr::unchecked(FEE_COLLECTION_ADDR.to_string())),
                    fees: FeeParams {
                        creation: Uint128::from(100_000u128),
                        reaction: Uint128::zero(),
                        link: Uint128::zero(),
                        text: Uint128::zero(),
                        tag: Uint128::zero(),
                        tip_pct: Uint128::zero(),
                    },
                },
                operator: Some(sender.clone()),
                root: NodeInitArgs {
                    parent_id: "".to_string(),
                    title: "Test thread".to_string(),
                    body: None,
                    links: None,
                    tags: None,
                    nsfw: None,
                },
            },
            Some(&sender),
            None,
        )?;

        if network.clone().eq(&networks::UNI_6) || network.clone().eq(&networks::JUNO_1) {
            let _ = chain.commit_any::<Any>(
                vec![juno_feeshare_msg(
                    cw_post_contract.addr_str()?,
                    sender.to_string(),
                    FEE_COLLECTION_ADDR.to_string(),
                )],
                None,
            );
        }
    } else {
        cw_post_contract.migrate_if_needed(&MigrateMsg {})?;
    }

    Ok(())
}

pub fn juno_feeshare_msg(
    contract_address: String,
    deployer_address: String,
    withdrawer_address: String,
) -> Any {
    Any {
        type_url: "/juno.feeshare.v1.MsgRegisterFeeShare".to_string(),
        value: Anybuf::new()
            .append_string(1, contract_address)
            .append_string(2, deployer_address)
            .append_string(3, withdrawer_address)
            .into_vec(),
    }
}

pub fn juno_update_feeshare_msg(
    contract_address: String,
    deployer_address: String,
    withdrawer_address: String,
) -> Any {
    Any {
        type_url: "/juno.feeshare.v1.MsgUpdateFeeShare".to_string(),
        value: Anybuf::new()
            .append_string(1, contract_address)
            .append_string(2, deployer_address)
            .append_string(3, withdrawer_address)
            .into_vec(),
    }
}

// pub fn set_archway_flat_fee_msg() {
//     let metadata_any: archway_proto::Any = MsgSetContractMetadata {
//         sender_address: daemon.sender().to_string(),
//         metadata: Some(ContractMetadata {
//             contract_address: contract.address().unwrap().to_string(),
//             owner_address: daemon.sender().to_string(),
//             rewards_address: daemon.sender().to_string(),
//         }),
//     }
//     .to_any()
//     .unwrap();

//     daemon
//         .commit_any::<prost_types::Any>(
//             vec![prost_types::Any {
//                 type_url: metadata_any.type_url,
//                 value: metadata_any.value,
//             }],
//             None,
//         )
//         .unwrap();

//     let flatfee_any: archway_proto::Any = MsgSetFlatFee {
//         sender_address: daemon.sender().to_string(),
//         contract_address: contract.address().unwrap().to_string(),
//         flat_fee_amount: Some(Coin {
//             amount: (150_000_000_000_000_000u128).to_string(),
//             denom: "aarch".to_string(),
//         }),
//     }
//     .to_any()
//     .unwrap();
// }
