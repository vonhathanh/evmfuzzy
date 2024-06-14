use libafl_bolts::current_nanos;


pub struct FuzzState<>
{
}

impl<> FuzzState<>
{
    /// Create a new [`FuzzState`] with default values
    pub fn new(lparam_seed: u64) -> Self {
        let mut seed: u64 = lparam_seed;
        if lparam_seed == 0 {
            seed = current_nanos();
        }
        Self {
        }
    }
}