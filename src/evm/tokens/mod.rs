pub mod v2_transformer;
pub mod v3_transformer;
pub mod weth_transformer;

use std::{cell::RefCell, rc::Rc};
use std::fmt::Debug;

use super::types::EVMAddress;

#[derive(Clone, Debug, Default)]
pub struct UniswapInfo {
    pub pool_fee: usize,
    pub router: Option<EVMAddress>,
}

#[derive(Clone)]
enum PairContextTy {
    Uniswap(Rc<RefCell<v2_transformer::UniswapPairContext>>),
    UniswapV3(Rc<RefCell<v3_transformer::UniswapV3PairContext>>),
    Weth(Rc<RefCell<weth_transformer::WethContext>>),
}

impl Debug for PairContextTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PairContextTy::Uniswap(ctx) => write!(f, "Uniswap({:?})", ctx.borrow()),
            PairContextTy::Weth(ctx) => write!(f, "Weth({:?})", ctx.borrow()),
            PairContextTy::UniswapV3(ctx) => write!(f, "UniswapV3({:?})", ctx.borrow()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PathContext {
    pub route: Vec<PairContextTy>,
}

#[derive(Clone, Debug, Default)]
pub struct TokenContext {
    pub swaps: Vec<PathContext>,
    pub is_weth: bool,
    pub weth_address: EVMAddress,
}
