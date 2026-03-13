pub mod analysis;
pub mod control;
pub mod curve;
pub mod demand;
pub mod link;
pub mod node;
pub mod options;
pub mod pattern;
pub mod report;
pub mod rule;

pub use control::Control;
pub use curve::Curve;
pub use demand::Demand;
pub use num_derive::FromPrimitive;
pub use num_traits::FromPrimitive;
pub use pattern::Pattern;
pub use report::ReportCallback;
pub use rule::Rule;

use crate::bindings::*;

/// Max ID Size
pub const MAX_ID_SIZE: EN_SizeLimits = EN_SizeLimits_EN_MAXID;
/// Max message size
pub const MAX_MSG_SIZE: EN_SizeLimits = EN_SizeLimits_EN_MAXMSG;

/// Max project title size. Taken from the EPANET C API source code.
pub const MAX_TITLE_SIZE: EN_SizeLimits = 79;


#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum ObjectType {
    Node = EN_ObjectType_EN_NODE as i32, // Nodes
    Link = EN_ObjectType_EN_LINK as i32, // Links
    TimePattern = EN_ObjectType_EN_TIMEPAT as i32, // Time patterns
    Curve = EN_ObjectType_EN_CURVE as i32, // Data curves
    Control = EN_ObjectType_EN_CONTROL as i32, // Simple controls
    Rule = EN_ObjectType_EN_RULE as i32, // Control rules
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum CountType {
    NodeCount = EN_CountType_EN_NODECOUNT as i32, // Number of nodes (junctions + tanks + reservoirs)
    TankCount = EN_CountType_EN_TANKCOUNT as i32, // Number of tanks and reservoirs
    LinkCount = EN_CountType_EN_LINKCOUNT as i32, // Number of links (pipes + pumps + valves)
    PatternCount = EN_CountType_EN_PATCOUNT as i32, // Number of time patterns
    CurveCount = EN_CountType_EN_CURVECOUNT as i32, // Number of data curves
    ControlCount = EN_CountType_EN_CONTROLCOUNT as i32, // Number of simple controls
    RuleCount = EN_CountType_EN_RULECOUNT as i32, // Number of rule-based controls
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum ActionCodeType {
    Unconditional = EN_ActionCodeType_EN_UNCONDITIONAL as i32, // Delete all controls and connecting links
    Conditional = EN_ActionCodeType_EN_CONDITIONAL as i32, // Cancel object deletion if it appears in controls or has connecting links
}
