use std::collections::HashMap;

use crate::evm::types::{EVMAddress, EVMU256};

pub struct ERC20Producer {
    pub balances: HashMap<(EVMAddress, EVMAddress), EVMU256>,
    pub balance_of: Vec<u8>,
}

impl ERC20Producer {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            balance_of: hex::decode("70a08231").unwrap()
        }
    }
}