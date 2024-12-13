use url::Url;

pub struct BeaconClient {
    rpc_url: Url,
}

impl BeaconClient {
    pub fn new(rpc_url: Url) -> Self {
        Self { rpc_url }
    }
}
