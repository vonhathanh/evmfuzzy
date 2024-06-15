use std::fmt::Formatter;
use std::fmt::Debug;

use libafl::schedulers::Scheduler;

use crate::{generic_vm::vm_executor::MAP_SIZE, scheduler::PowerABIScheduler};

use super::{types::{EVMFuzzState, EVMU256}, vm::EVMState};

pub static mut JMP_MAP: [u8; MAP_SIZE] = [0; MAP_SIZE];

// dataflow
pub static mut READ_MAP: [bool; MAP_SIZE] = [false; MAP_SIZE];
pub static mut WRITE_MAP: [u8; MAP_SIZE] = [0; MAP_SIZE];

// cmp
pub static mut CMP_MAP: [EVMU256; MAP_SIZE] = [EVMU256::MAX; MAP_SIZE];

#[allow(clippy::type_complexity)]
pub struct FuzzHost<>
{
    pub evmstate: EVMState,
    // [EIP-1153[(https://eips.ethereum.org/EIPS/eip-1153) transient storage that is discarded after every transactions
    // pub transient_storage: HashMap<(EVMAddress, EVMU256), EVMU256>,
    // // these are internal to the host
    // pub env: Env,
    // pub code: HashMap<EVMAddress, Arc<BytecodeLocked>>,
    // pub hash_to_address: HashMap<[u8; 4], HashSet<EVMAddress>>,
    // pub address_to_hash: HashMap<EVMAddress, Vec<[u8; 4]>>,
    // pub _pc: usize,
    // pub pc_to_addresses: HashMap<(EVMAddress, usize), HashSet<EVMAddress>>,
    // pub pc_to_create: HashMap<(EVMAddress, usize), usize>,
    // pub pc_to_call_hash: HashMap<(EVMAddress, usize, usize), HashSet<Vec<u8>>>,
    // pub middlewares_enabled: bool,
    // // If you use RefCell, modifying middlewares during execution will cause a panic
    // // because the executor borrows middlewares over its entire lifetime.
    // pub middlewares: RwLock<Vec<Rc<RefCell<dyn Middleware<SC>>>>>,

    // pub coverage_changed: bool,

    // pub flashloan_middleware: Option<Rc<RefCell<Flashloan>>>,

    // pub middlewares_latent_call_actions: Vec<CallMiddlewareReturn>,

    pub scheduler: PowerABIScheduler<>,

    // // controlled by onchain module, if sload cant find the slot, use this value
    // pub next_slot: EVMU256,

    // pub access_pattern: Rc<RefCell<AccessPattern>>,

    // pub bug_hit: bool,
    // pub current_typed_bug: Vec<(String, (EVMAddress, usize))>,
    // pub call_count: u32,

    // #[cfg(feature = "print_logs")]
    // pub logs: HashSet<u64>,
    // // set_code data
    // pub setcode_data: HashMap<EVMAddress, Bytecode>,
    // // selftdestruct
    // pub current_self_destructs: Vec<(EVMAddress, usize)>,
    // // arbitrary calls
    // pub current_arbitrary_calls: Vec<(EVMAddress, EVMAddress, usize)>,
    // // integer_overflow
    // pub current_integer_overflow: HashSet<(EVMAddress, usize, &'static str)>,
    // // relations file handle
    // relations_file: std::fs::File,
    // // Filter duplicate relations
    // relations_hash: HashSet<u64>,
    // /// Randomness from inputs
    // pub randomness: Vec<u8>,
    /// workdir
    pub work_dir: String,
    // /// custom SpecId
    // pub spec_id: SpecId,
    // /// Precompiles
    // pub precompiles: Precompiles,

    // /// All SSTORE PCs that are for mapping (i.e., writing to multiple storage
    // /// slots)
    // pub mapping_sstore_pcs: HashSet<(EVMAddress, usize)>,
    // pub mapping_sstore_pcs_to_slot: HashMap<(EVMAddress, usize), HashSet<EVMU256>>,

    // /// For future continue executing when control leak happens
    // pub leak_ctx: Vec<SinglePostExecution>,

    // pub jumpi_trace: usize,

    // /// Depth of call stack
    // pub call_depth: u64,
    // /// Prank information
    // pub prank: Option<Prank>,
    // /// Expected revert information
    // pub expected_revert: Option<ExpectedRevert>,
    // /// Expected emits
    // pub expected_emits: VecDeque<ExpectedEmit>,
    // /// Expected calls
    // pub expected_calls: ExpectedCallTracker,
    // /// Assert failed message for the cheatcode
    // pub assert_msg: Option<String>,
}

impl FuzzHost {
    pub fn new(scheduler: PowerABIScheduler<String>, workdir: String) -> Self {
        Self {
            evmstate: EVMState::new(),
            scheduler: scheduler,
            work_dir: workdir
        }
    }
}


impl Debug for FuzzHost
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FuzzHost")
            .field("data", &self.evmstate)
            .finish()
    }
}