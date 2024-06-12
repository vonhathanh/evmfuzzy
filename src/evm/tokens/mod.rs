pub mod v2_transfomer;
pub mod v3_transformer;
pub mod weth_transformer;

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::types::EVMAddress;


#[derive(Clone, Debug, Default)]
pub struct PathContext {
    pub route: Vec<PairContextTy>,
}

#[derive(Clone)]
enum PairContextTy {
    Uniswap(Rc<RefCell<v2_transfomer::UniswapPairContext>>),
    UniswapV3(Rc<RefCell<v3_transformer::UniswapV3PairContext>>),
    Weth(Rc<RefCell<weth_transformer::WethContext>>)
}

impl Debug for PairContextTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uniswap(ctx) => write!(f, "Uniswap({:?})", ctx.borrow()),
            Self::UniswapV3(ctx) => write!(f, "UniswapV3({:?})", ctx.borrow()),
            Self::Weth(ctx) => write!(f, "Weth({:?})", ctx.borrow()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TokenContext {
    pub swaps: Vec<PathContext>,
    pub is_weth: bool,
    pub weth_address: EVMAddress,
}

#[derive(Clone, Debug, Default)]
pub struct UniswapInfo {
    pub pool_fee: usize,
    pub router: Option<EVMAddress>,
}
