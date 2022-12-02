/// Type alias for exception status flags.
pub type Flags = u32;

/// Type alias for rounding modes.
pub type Rounding = u32;

pub const DEC_FE_INVALID: Flags = 0x01;

pub const BID_INVALID_EXCEPTION: Flags = DEC_FE_INVALID;

// rounding modes
// pub const BID_ROUNDING_TO_NEAREST: Rounding = 0x00000;
// pub const BID_ROUNDING_DOWN: Rounding = 0x00001;
// pub const BID_ROUNDING_UP: Rounding = 0x00002;
// pub const BID_ROUNDING_TO_ZERO: Rounding = 0x00003;
// pub const BID_ROUNDING_TIES_AWAY: Rounding = 0x00004;
