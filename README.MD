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

