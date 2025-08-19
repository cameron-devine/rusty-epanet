mod analysis;
pub mod curve;
pub mod types;
pub mod rule;
pub mod control;
pub mod node;

pub use curve::Curve;
pub use rule::Rule;
pub use control::Control;

use crate::bindings::*;

/// Max ID Size
pub const MAX_ID_SIZE: EN_SizeLimits = EN_SizeLimits_EN_MAXID;
/// Max message size
pub const MAX_MSG_SIZE: EN_SizeLimits = EN_SizeLimits_EN_MAXMSG;

/// Max project title size. Taken from the EPANET C API source code.
pub const MAX_TITLE_SIZE: EN_SizeLimits = 79;