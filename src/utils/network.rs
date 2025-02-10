pub enum Network {
    KITEAI,
}

#[derive(Clone)]
pub struct RPC {
    pub chain_id: u64,
    pub rpc_url: &'static str,
    pub explorer: &'static str,
    pub symbol: &'static str,
}

impl Network {
    pub fn get_rpc_details(&self) -> RPC {
        match self {
            Network::KITEAI => RPC {
                chain_id: 2368,
                rpc_url: "https://rpc-testnet.gokite.ai",
                explorer: "https://testnet.kitescan.ai/",
                symbol: "KITE",
            },
        }
    }
}
