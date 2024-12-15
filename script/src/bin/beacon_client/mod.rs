use std::time::Duration;

use lighthouse_eth2::{types::StateId, BeaconNodeHttpClient, SensitiveUrl, Timeouts};
use lighthouse_types::{MainnetEthSpec, Slot};

pub struct BeaconClient {
    client: BeaconNodeHttpClient,
}

impl BeaconClient {
    pub fn new(rpc_url: String) -> Result<Self, String> {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::parse(&rpc_url)
                .map_err(|e| format!("Failed to parse beacon http server: {:?}", e))?,
            Timeouts::set_all(Duration::from_secs(30)),
        );
        Ok(Self { client })
    }

    pub async fn get_beacon_state(&self, slot: u64) -> Result<Slot, String> {
        let beacon_state = self
            .client
            .get_debug_beacon_states::<MainnetEthSpec>(StateId::Slot(slot.into()))
            .await
            .unwrap()
            .expect("Failed to get beacon state");

        println!("slot: {:?}", beacon_state.data.slot());

        Ok(beacon_state.data.slot())
    }
}
