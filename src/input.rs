use revm_primitives::Env;

use crate::evm::{abi::BoxedABI, types::{EVMAddress, EVMU256}};

/// A trait for VM inputs that are sent to any smart contract VM
pub trait EVMInputT {}

/// EVM Input
pub struct EVMInput {
    /// Caller address
    pub caller: EVMAddress,

    /// Contract address
    pub contract: EVMAddress,

    /// Input data in ABI format
    pub data: Option<BoxedABI>,

    /// Transaction value in wei
    pub txn_value: Option<EVMU256>,

    /// Environment (block, timestamp, etc.)
    pub env: Env,

    /// Additional random bytes for mutator
    pub randomness: Vec<u8>,

    /// Execute the transaction multiple times
    pub repeat: usize,
}