use revm_primitives::{ruint::aliases::B160, U256};

use crate::state::FuzzState;

pub type EVMAddress = B160;
pub type EVMU256 = U256;

pub type EVMFuzzState = FuzzState<>;