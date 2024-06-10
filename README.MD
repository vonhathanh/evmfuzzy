# Install
- Install rust
- clone the repo and run `cargo build`

# TODOs
- To fuzzy testing the contract we must: load the contract abi on etherscan or from solidity file, abi file locally.
- Create demo contract fuzzy using libafl
- Support fuzzy multiple contracts concurrently

# LibAFL main components that need to be override
- Observer
- Executor
- Feedback
- Input
- Corpus
- Mutator
- Generator
- Stage