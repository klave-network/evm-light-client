use crate::light_client_cli::src::{chain::Chain, client::LightClient, context::Context};
use anyhow::Result;
use clap::Parser;
use crate::lodestar_rpc::src::types::GenesisData;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Parser, PartialEq, Serialize, Deserialize)]
pub struct PersistCommand {
    #[clap(long = "bootstrap_info", help = "Bootstrap Info")]
    pub bootstrap_info: Option<String>,
    #[clap(long = "state_info", help = "State Info")]
    pub state_info: Option<String>,
    #[clap(long = "genesis_info", help = "Genesis Info")]
    pub genesis_info: Option<String>,
}

impl PersistCommand {
    pub fn run<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const SYNC_COMMITTEE_SIZE: usize,
    >(
        self,
        ctx: Context<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES, SYNC_COMMITTEE_SIZE>
    ) -> Result<()> {
        let chain = Chain::new(ctx.beacon_endpoint());
        let genesis_info = match self.genesis_info {
            Some(genesis_info) => match serde_json::from_str::<GenesisData>(&genesis_info) {
                Ok(genesis) => genesis,
                Err(e) => {
                    println!("Invalid genesis info: {}", e);
                    return Ok(());
                }
            },
            None => {
                println!("Genesis info is required");                
                return Ok(());
            }
        };
        let lc = LightClient::new(
            ctx,
            chain,            
            genesis_info.genesis_time,
            genesis_info.genesis_validators_root,
            None,
        );

        let _ = match self.bootstrap_info {
            Some(bootstrap_info) => lc.store_boostrap(match serde_json::from_str(&bootstrap_info) {
                Ok(bootstrap) => bootstrap,
                Err(e) => {
                    eprintln!("Invalid bootstrap info: {}", e);
                    return Ok(());
                }
            }),
            None => Ok(()),
        };
        let _ = match self.state_info {
            Some(state_info) => lc.store_light_client_state(match serde_json::from_str(&state_info) {
                Ok(state) => state,
                Err(e) => {
                    eprintln!("Invalid state info: {}", e);
                    return Ok(());
                }
            }),
            None => Ok(()),
        };
        lc.store_genesis(&genesis_info)?;
        klave::notifier::send_string("Light client genesis, boostrap and state info persisted");
        Ok(())
    }
}
