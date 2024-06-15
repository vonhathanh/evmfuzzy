use std::marker::PhantomData;

/// A scheduler that drops inputs (or VMState) based on a voting mechanism
#[derive(Debug, Clone)]
pub struct SortedDroppingScheduler<S> {
    phantom: PhantomData<S>,
}

impl<S> Default for SortedDroppingScheduler<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> SortedDroppingScheduler<S> {
    /// Create a new SortedDroppingScheduler
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PowerABIScheduler<S> {
    phantom: PhantomData<S>,
}

impl<S> Default for PowerABIScheduler<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> PowerABIScheduler<S> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}