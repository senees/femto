/// Type alias for exception status flags.
pub type Flags = u32;

/// Type alias for rounding modes.
pub type Rounding = u32;

pub const BID_EXACT_STATUS: Flags = 0x00000000;

pub const DEC_FE_INVALID: Flags = 0x01;
pub const _DEC_FE_UNNORMAL: Flags = 0x02;
pub const _DEC_FE_DIVBYZERO: Flags = 0x04;
pub const _DEC_FE_OVERFLOW: Flags = 0x08;
pub const DEC_FE_UNDERFLOW: Flags = 0x10;
pub const DEC_FE_INEXACT: Flags = 0x20;

pub const BID_INEXACT_EXCEPTION: Flags = DEC_FE_INEXACT;
pub const BID_INVALID_EXCEPTION: Flags = DEC_FE_INVALID;
pub const BID_UNDERFLOW_EXCEPTION: Flags = DEC_FE_UNDERFLOW;

// rounding modes
pub const BID_ROUNDING_TO_NEAREST: Rounding = 0x00000;
pub const BID_ROUNDING_DOWN: Rounding = 0x00001;
pub const BID_ROUNDING_UP: Rounding = 0x00002;
pub const BID_ROUNDING_TO_ZERO: Rounding = 0x00003;
pub const BID_ROUNDING_TIES_AWAY: Rounding = 0x00004;
