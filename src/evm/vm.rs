use libafl::schedulers::Scheduler;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::generic_vm::vm_state::VMStateT;

use super::{host::FuzzHost, types::{EVMAddress, EVMFuzzState, EVMU256}};


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EVMState {
    /// State of the EVM, which is mapping of EVMU256 slot to EVMU256 value for
    /// each contract
    pub state: HashMap<EVMAddress, HashMap<EVMU256, EVMU256>>,

    /// Balance of addresses
    pub balance: HashMap<EVMAddress, EVMU256>,

    /// Post execution context
    /// If control leak happens, we add the post execution context to the VM
    /// state, which contains all information needed to continue execution.
    ///
    /// There can be more than one [`PostExecutionCtx`] when the control is
    /// leaked again on the incomplete state (i.e., double+ reentrancy)
    // pub post_execution: Vec<PostExecutionCtx>,

    /// Flashloan information
    /// (e.g., how much flashloan is taken, and how much tokens are liquidated)
    // #[serde(skip)]
    // pub flashloan_data: FlashloanData,

    /// Is bug() call in Solidity hit?
    #[serde(skip)]
    pub bug_hit: bool,
    /// selftdestruct() call in Solidity hit?
    #[serde(skip)]
    pub self_destruct: HashSet<(EVMAddress, usize)>,
    /// bug type call in solidity type
    #[serde(skip)]
    pub typed_bug: HashSet<(String, (EVMAddress, usize))>,
    #[serde(skip)]
    pub arbitrary_calls: HashSet<(EVMAddress, EVMAddress, usize)>,
    // integer overflow in sol
    #[serde(skip)]
    pub integer_overflow: HashSet<(EVMAddress, usize, &'static str)>,
    // #[serde(skip)]
    // pub reentrancy_metadata: ReentrancyData,
    // #[serde(skip)]
    // pub swap_data: SwapData,
}

impl EVMState {
    /// Create a new EVM state, containing empty state, no post execution
    /// context
    pub(crate) fn new() -> Self {
        Default::default()
    }
}

/// EVM executor, wrapper of revm
#[derive(Debug, Clone)]
pub struct EVMExecutor<VS, CI, SC>
where
    VS: VMStateT,
    SC: Scheduler<State = EVMFuzzState> + Clone,
{
    /// Host providing the blockchain environment (e.g., writing/reading
    /// storage), needed by revm
    pub host: FuzzHost,
    /// [Depreciated] Deployer address
    pub deployer: EVMAddress,
    /// Known arbitrary (caller,pc)
    pub _known_arbitrary: HashSet<(EVMAddress, usize)>,
    phandom: PhantomData<(EVMInput, VS, CI)>,
}
