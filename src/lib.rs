pub mod avm;
pub mod encoding;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AvmError {
    #[error("Empty program")]
    EmptyProgram,
    #[error("Empty stack")]
    EmptyStack,
    #[error("Invalid AVM Version: {0:#04x}")]
    InvalidAvmVerison(u8),
    #[error("Unknown opcode: {0:#04x}")]
    UnknownOpcode(u8),
    #[error("Invalid varint")]
    InvalidVarUint64,
    #[error("Invalid varbytes")]
    InvalidVarBytes,
    #[error("Program counter out of bounds")]
    PcOutOfBounds,
    #[error("Integer overflow")]
    IntegerOverflow,
    #[error("Integer underflow")]
    IntegerUnderflow,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Invalid stack access")]
    InvalidStackAccess,
    #[error("Incompatible types (got {0}, expected {0})")]
    IncompatibleTypes(&'static str, &'static str),
    #[error("Byte slice exceeds length 4096")]
    BytesTooLong,
    #[error("err opcode executed")]
    ErrOpCode,
    #[error("Integer constant {0} out of range {1}")]
    IntcOutOfRange(usize, usize),
    #[error("Byte constant {0} out of range {1}")]
    BytecOutOfRange(usize, usize),
    #[error("Byte slice (length {0}) too long for btoi conversion")]
    BtoiTooLong(usize),
    #[error("Scratch position {0} out of bounds")]
    ScratchAccessOutOfBounds(usize),
    #[error("Assertion failed at program counter {0}")]
    AssertionFailed(usize),
    #[error("Impossible to access substring from positions {0} to {1} (length {2})")]
    InvalidSubstringAccess(usize, usize, usize),
}
