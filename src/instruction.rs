use std::fmt::{self, Display, Formatter};

use ethnum::U256;

/// An EVM instruction.
pub enum Instruction {
    /// Halts execution.
    Stop,
    /// Addition operation.
    Add,
    /// Multiplication operation.
    Mul,
    /// Subtraction operation.
    Sub,
    /// Integer division operation.
    Div,
    /// Signed integer division operation (truncated).
    Sdiv,
    /// Modulo remainder operation.
    Mod,
    /// Signed modulo remainder operation.
    Smod,
    /// Modulo addition operation.
    AddMod,
    /// Modulo multiplication operation.
    MulMod,
    /// Exponential operation.
    Exp,
    /// Extend length of two's complement signed integer.
    SignExtend,
    /// Less-than comparison.
    Lt,
    /// Greater-than comparison.
    Gt,
    /// Signed less-than comparison.
    Slt,
    /// Signed greater-than comparison.
    Sgt,
    /// Equality comparison.
    Eq,
    /// Simple not operator.
    IsZero,
    /// Bitwise AND operation.
    And,
    /// Bitwise OR operation.
    Or,
    /// Bitwise XOR operation.
    Xor,
    /// Bitwise NOT operation.
    Not,
    /// Retrieve single byte from word.
    Byte,
    /// Shift Left.
    Shl,
    /// Logical Shift Right.
    Shr,
    /// Arithmetic Shift Right.
    Sar,
    /// Compute Keccak-256 hash.
    Keccak256,
    /// Get address of currently executing account.
    Address,
    /// Get balance of the given account.
    Balance,
    /// Get execution origination address.
    Origin,
    /// Get caller address.
    Caller,
    /// Get deposited value by the instruction/transaction responsible for this
    /// execution.
    CallValue,
    /// Get input data of current environment.
    CallDataLoad,
    /// Get size of input data in current environment.
    CallDataSize,
    /// Copy input data in current environment to memory.
    CallDataCopy,
    /// Get size of code running in current environment.
    CodeSize,
    /// Copy code running in current environment to memory.
    CodeCopy,
    /// Get price of gas in current environment.
    GasPrice,
    /// Get size of an account's code.
    ExtCodeSize,
    /// Copy an account's code to memory.
    ExtCodeCopy,
    /// Pushes the size of the return data buffer onto the stack.
    ReturnDataSize,
    /// Copies data from the return data buffer to memory.
    ReturnDataCopy,
    /// Returns the keccak256 hash of a contract's code.
    ExtCodeHash,
    /// Get the hash of one of the 256 most recent complete blocks.
    BlockHash,
    /// Get the block's beneficiary address.
    Coinbase,
    /// Get the block's timestamp.
    Timestamp,
    /// Get the block's number.
    Number,
    /// Get the block's difficulty.
    Difficulty,
    /// Get the block's gas limit.
    GasLimit,
    /// Returns the current chain’s EIP-155 unique identifier.
    ChainId,
    /// Remove word from stack.
    Pop,
    /// Load word from memory.
    MLoad,
    /// Save word to memory.
    MStore,
    /// Save byte to memory.
    MStore8,
    /// Load word from storage.
    SLoad,
    /// Save word to storage.
    SStore,
    /// Alter the program counter.
    Jump,
    /// Conditionally alter the program counter.
    JumpI,
    /// Get the value of the program counter prior to the increment.
    GetPc,
    /// Get the size of active memory in bytes.
    MSize,
    /// Get the amount of available gas, including the corresponding reduction
    /// the amount of available gas.
    Gas,
    /// Mark a valid destination for jumps.
    JumpDest(usize),
    /// Place value on the stack.
    Push(u8, U256),
    /// Duplicate n-th stack item.
    Dup(u8),
    /// Exchange 1st and (n+1)-th stack items.
    Swap(u8),
    /// Append log record with n topics.
    Log(u8),
    /// Create a new account with associated code.
    Create,
    /// Message-call into an account.
    Call,
    /// Message-call into this account with alternative account's code.
    CallCode,
    /// Halt execution returning output data.
    Return,
    /// Message-call into this account with an alternative account's code, but
    /// persisting into this account with an alternative account's code.
    DelegateCall,
    /// Create a new account and set creation address to
    /// `address(keccak256(sender || keccak256(init code)))`.
    Create2,
    /// Similar to [`Call`], but does not modify state.
    StaticCall,
    /// Stop execution and revert state changes, without consuming all provided
    /// gas and providing a reason.
    Revert,
    /// Designated invalid instruction.
    Invalid,
    /// Halt execution and register account for later deletion.
    SelfDestruct,
    /// Unknown binary data.
    Unknown(u8),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Instruction::*;
        match self {
            Stop => f.write_str("stop"),
            Add => f.write_str("add"),
            Mul => f.write_str("mul"),
            Sub => f.write_str("sub"),
            Div => f.write_str("div"),
            Sdiv => f.write_str("sdiv"),
            Mod => f.write_str("mod"),
            Smod => f.write_str("smod"),
            AddMod => f.write_str("addmod"),
            MulMod => f.write_str("mulmod"),
            Exp => f.write_str("exp"),
            SignExtend => f.write_str("signextend"),
            Lt => f.write_str("lt"),
            Gt => f.write_str("gt"),
            Slt => f.write_str("slt"),
            Sgt => f.write_str("sgt"),
            Eq => f.write_str("eq"),
            IsZero => f.write_str("iszero"),
            And => f.write_str("and"),
            Or => f.write_str("or"),
            Xor => f.write_str("xor"),
            Not => f.write_str("not"),
            Byte => f.write_str("byte"),
            Shl => f.write_str("shl"),
            Shr => f.write_str("shr"),
            Sar => f.write_str("sar"),
            Keccak256 => f.write_str("keccak256"),
            Address => f.write_str("address"),
            Balance => f.write_str("balance"),
            Origin => f.write_str("origin"),
            Caller => f.write_str("caller"),
            CallValue => f.write_str("callvalue"),
            CallDataLoad => f.write_str("calldataload"),
            CallDataSize => f.write_str("calldatasize"),
            CallDataCopy => f.write_str("calldatacopy"),
            CodeSize => f.write_str("codesize"),
            CodeCopy => f.write_str("codecopy"),
            GasPrice => f.write_str("gasprice"),
            ExtCodeSize => f.write_str("extcodesize"),
            ExtCodeCopy => f.write_str("extcodecopy"),
            ReturnDataSize => f.write_str("returndatasize"),
            ReturnDataCopy => f.write_str("returndatacopy"),
            ExtCodeHash => f.write_str("extcodehash"),
            BlockHash => f.write_str("blockhash"),
            Coinbase => f.write_str("coinbase"),
            Timestamp => f.write_str("timestamp"),
            Number => f.write_str("number"),
            Difficulty => f.write_str("difficulty"),
            GasLimit => f.write_str("gaslimit"),
            ChainId => f.write_str("chainid"),
            Pop => f.write_str("pop"),
            MLoad => f.write_str("mload"),
            MStore => f.write_str("mstore"),
            MStore8 => f.write_str("mstore8"),
            SLoad => f.write_str("sload"),
            SStore => f.write_str("sstore"),
            Jump => f.write_str("jump"),
            JumpI => f.write_str("jumpi"),
            GetPc => f.write_str("getpc"),
            MSize => f.write_str("msize"),
            Gas => f.write_str("gas"),
            JumpDest(offset) => write!(f, "jumpdest :{:x}", offset),
            Push(size, value) => write!(f, "push{} {:0w$x}", size, value, w = (size * 2) as usize),
            Dup(slot) => write!(f, "dup{}", slot),
            Swap(slot) => write!(f, "swap{}", slot),
            Log(topics) => write!(f, "log{}", topics),
            Create => f.write_str("create"),
            Call => f.write_str("call"),
            CallCode => f.write_str("callcode"),
            Return => f.write_str("return"),
            DelegateCall => f.write_str("delegatecall"),
            Create2 => f.write_str("create2"),
            StaticCall => f.write_str("staticcall"),
            Revert => f.write_str("revert"),
            Invalid => f.write_str("invalid"),
            SelfDestruct => f.write_str("selfdestruct"),
            Unknown(op) => write!(f, "?{:02x}?", op),
        }
    }
}
