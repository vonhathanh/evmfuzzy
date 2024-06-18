use super::v2_transformer::UniswapPairContext;

#[derive(Debug)]
pub struct UniswapV3PairContext {
    pub fee: u32,
    pub inner: UniswapPairContext,
}