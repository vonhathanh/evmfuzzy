use crate::evm::types::EVMAddress;

#[derive(Clone, Debug, Default)]
pub struct WethContext {
    pub weth_address: EVMAddress,
}