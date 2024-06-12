use std::{cell::RefCell, rc::Rc};

use revm_primitives::HashMap;

use crate::evm::{producers::erc20::ERC20Producer, types::{EVMAddress, EVMU256}};
use crate::evm::tokens::TokenContext;

pub struct IERC20OracleFlashloan {
    pub balance_of: Vec<u8>,
    pub known_tokens: HashMap<EVMAddress, TokenContext>,
    pub known_pair_reserve_slot: HashMap<EVMAddress, EVMU256>,
    pub erc20_producer: Rc<RefCell<ERC20Producer>>,
}

impl IERC20OracleFlashloan {
    pub fn new(erc20_producer: Rc<RefCell<ERC20Producer>>) -> Self {
        Self {
            balance_of: hex::decode("70a08231").unwrap(),
            known_tokens: HashMap::new(),
            known_pair_reserve_slot: HashMap::new(),
            erc20_producer,
        }
    }
}