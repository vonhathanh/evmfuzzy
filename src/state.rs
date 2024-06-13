use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::generic_vm::vm_state::VMStateT;

/// The global state of ItyFuzz, containing all the information needed for
/// fuzzing Implements LibAFL's [`State`] trait and passed to all the fuzzing
/// components as a reference
///
/// VI: The type of input
/// VS: The type of VMState
/// Loc: The type of the call target
/// Addr: The type of the address (e.g., H160 address for EVM)
/// Out: The type of the output
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "Addr: Serialize + DeserializeOwned, Out: Serialize + DeserializeOwned")]
pub struct FuzzState<VI, VS, Loc, Addr, Out, CI>
where
    VS: Default + VMStateT,
    VI: VMInputT<VS, Loc, Addr, CI> + Input,
    Addr: Debug + Serialize + DeserializeOwned + Clone,
    Loc: Debug + Serialize + DeserializeOwned + Clone,
    Out: Default + Into<Vec<u8>> + Clone,
    CI: Serialize + DeserializeOwned + Debug + Clone + ConciseSerde,
{
    /// InfantStateState wraps the infant state corpus with [`State`] trait so
    /// that it is easier to use
    #[serde(deserialize_with = "InfantStateState::deserialize")]
    pub infant_states_state: InfantStateState<Loc, Addr, VS, CI>,

    /// The input corpus
    #[cfg(feature = "evaluation")]
    #[serde(deserialize_with = "OnDiskCorpus::deserialize")]
    txn_corpus: OnDiskCorpus<VI>,
    #[cfg(not(feature = "evaluation"))]
    #[serde(deserialize_with = "InMemoryCorpus::deserialize")]
    txn_corpus: InMemoryCorpus<VI>,

    /// The solution corpus
    #[serde(deserialize_with = "OnDiskCorpus::deserialize")]
    solutions: OnDiskCorpus<VI>,

    /// Amount of total executions
    executions: usize,

    /// Metadata of the state, required for implementing [HasMetadata] and
    /// [HasNamedMetadata] trait
    metadata: SerdeAnyMap,
    named_metadata: NamedSerdeAnyMap,

    /// Current input index, used for concolic execution
    current_input_idx: usize,

    /// The current execution result
    #[serde(deserialize_with = "ExecutionResult::deserialize")]
    execution_result: ExecutionResult<Loc, Addr, VS, Out, CI>,

    /// Caller and address pools, required for implementing [`HasCaller`] trait
    pub callers_pool: Vec<Addr>,
    pub addresses_pool: Vec<Addr>,

    /// Random number generator, required for implementing [`HasRand`] trait
    pub rand_generator: RomuDuoJrRand,

    /// Maximum size for input, required for implementing [`HasMaxSize`] trait,
    /// used mainly for limiting the size of the arrays for ETH ABI
    pub max_size: usize,

    /// Mapping between function hash with the contract addresses that have the
    /// function, required for implementing [`HasHashToAddress`] trait
    pub hash_to_address: std::collections::HashMap<[u8; 4], HashSet<EVMAddress>>,

    /// The last time we reported progress (if available/used).
    /// This information is used by fuzzer `maybe_report_progress` and updated
    /// by event_manager.
    last_report_time: Option<Duration>,

    pub interesting_signatures: Vec<[u8; 4]>,

    pub sig_to_addr_abi_map: std::collections::HashMap<[u8; 4], (EVMAddress, BoxedABI)>,

    pub phantom: std::marker::PhantomData<(VI, Addr)>,
}
