# Install
- Install rust
- clone the repo and run `cargo build`

# TODOs
- To fuzzy testing the contract we must: load the contract abi on etherscan or from solidity file, abi file locally.
- Create demo contract fuzzy using libafl
- Support fuzzy multiple contracts concurrently

# LibAFL main components that need to be override
- State
- Observer
- Executor
- Feedback
- Input
  - Transaction struct (caller, contract address, tx data, value, nonce...)
- Corpuses: 
  - Normal Corpus: each testcase is a tuple of (s, t) where s is the db state, t is the Transactiion Input
  - Infant state corpus: stores only interesting states for futher exploration
- Mutator
- Generator
- Stage

# Design
- Input: 
  - Smart contract bytecodes + ABI (for EVMHost, Executor...)
  - Transactions: contract calls (bytes-encoded)
- Output: execution result + state changes
-> Define VMInput trait first

# Main components:
- Producers: has Producer trait that provides functions needed to produce data for the oracle
  - ERC20Procuder

- Oracles: implement Oracle trait: provides functions needed to implement an oracle
  - Transition function, called everytime after non-reverted execution
  - Oracle function, called everytime after non-reverted execution returns Some(bug_idx) if the oracle is violated
  - Implementer: IERC20OracleFlashloan,...
  - Oracle types: ERC20, Erentrancy...

- EVMFuzzState: global state of ItyFuzz, stores input corpus, solution corpus, and some other metadata...

- OffchainArtifact: store contract bytecode, abi, source codes... Uses for local/offchain testing

- ContractLoader:  hold ContractInfo, ABIInfo and SetupData. It can get/download contracts from onchain or OffchainArtifact

- Config: hold various config options

- EVMState: State of the EVM, which is mapping of EVMU256 slot to EVMU256 value for each contract. Also hold balance of addresses

- EVMInput: input of the EVM, contains input_type, caller, contract address, data (ABI)

- FuzzHost: where the real execution start, contains: EVMState, transient_storage, code map, middlewares and a scheduler

- EVMExecutor: wrapper of revm, contains FuzzHost, deployer address and known arbitrary (caller,pc)

- FuzzExecutor: Wrapper of smart contract VM, which implements LibAFL [`Executor`]

- ItyFuzzer: fuzzer that implements ItyFuzz logic using LibAFL's [`Fuzzer`] trait

# Main fuzzer flow
- Init monitor, event manager,scheduler, observer, feedback, mutator
- Init jmps, cmps.. mapping
- Add middlewares to fuzzhost
- Init evm executor, fuzzer
- Init corpus
