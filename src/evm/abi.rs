use std::any::Any;

pub enum ABILossyType {
    /// All 256-bit types (uint8, uint16, uint32, uint64, uint128, uint256,
    /// address...)
    T256,
    /// All array types (X[], X[n], (X,Y,Z))
    TArray,
    /// All dynamic types (string, bytes...)
    TDynamic,
    /// Empty type (nothing)
    TEmpty,
    /// Unknown type (e.g., those we don't know ABI, it can be any type)
    TUnknown,
}

/// Cloneable trait object, to support serde serialization
pub trait CloneABI {
    fn clone_box(&self) -> Box<dyn ABI>;
}


pub trait ABI: CloneABI {
    /// Is the args static (i.e., fixed size)
    fn is_static(&self) -> bool;
    /// Get the ABI-encoded bytes of args
    fn get_bytes(&self) -> Vec<u8>;
    /// Get the ABI type of args
    fn get_type(&self) -> ABILossyType;
    /// Set the bytes to args, used for decoding
    fn set_bytes(&mut self, bytes: Vec<u8>) -> bool;
    /// Convert args to string (for debugging)
    fn to_string(&self) -> String;
    fn as_any(&mut self) -> &mut dyn Any;
    /// Get the size of args
    fn get_size(&self) -> usize;
}

pub struct BoxedABI {
    /// ABI wrapper
    // #[serde(with = "serde_traitobject")]
    pub b: Box<dyn ABI>,
    /// Function hash, if it is 0x00000000, it means the function hash is not
    /// set or this is to resume execution from a previous control leak
    pub function: [u8; 4],
}