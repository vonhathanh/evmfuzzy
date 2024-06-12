use super::v2_transfomer::UniswapPairContext;

#[derive(Debug)]
pub struct UniswapV3PairContext {
    pub fee: u32,
    pub inner: UniswapPairContext,
}
