use std::sync::Arc;

use crate::evm::types::{EVMAddress, EVMU256};

use super::UniswapInfo;

#[derive(Clone, Debug, Default)]
pub struct UniswapPairContext {
    pub pair_address: EVMAddress,
    pub in_token_address: EVMAddress,
    pub next_hop: EVMAddress,
    pub side: u8,
    pub uniswap_info: Arc<UniswapInfo>,
    pub initial_reserves: (EVMU256, EVMU256),
}