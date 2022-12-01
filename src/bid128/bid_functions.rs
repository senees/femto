/// Type alias for exception status flags.
pub type ExceptionStatusFlag = u32;

pub const DEC_FE_INVALID: ExceptionStatusFlag = 0x01;

pub const BID_INVALID_EXCEPTION: ExceptionStatusFlag = DEC_FE_INVALID;
