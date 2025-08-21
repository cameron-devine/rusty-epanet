pub mod analysis;
pub mod control;
pub mod curve;
pub mod demand;
pub mod link;
pub mod node;
pub mod options;
pub mod rule;

pub use control::Control;
pub use curve::Curve;
use enum_primitive::*;
pub use rule::Rule;

use crate::bindings::*;

/// Max ID Size
pub const MAX_ID_SIZE: EN_SizeLimits = EN_SizeLimits_EN_MAXID;
/// Max message size
pub const MAX_MSG_SIZE: EN_SizeLimits = EN_SizeLimits_EN_MAXMSG;

/// Max project title size. Taken from the EPANET C API source code.
pub const MAX_TITLE_SIZE: EN_SizeLimits = 79;

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum ObjectType {
    Node = EN_ObjectType_EN_NODE, // Nodes
    Link = EN_ObjectType_EN_LINK, // Links
    TimePattern = EN_ObjectType_EN_TIMEPAT, // Time patterns
    Curve = EN_ObjectType_EN_CURVE, // Data curves
    Control = EN_ObjectType_EN_CONTROL, // Simple controls
    Rule = EN_ObjectType_EN_RULE, // Control rules
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum CountType {
    NodeCount = EN_CountType_EN_NODECOUNT, // Number of nodes (junctions + tanks + reservoirs)
    TankCount = EN_CountType_EN_TANKCOUNT, // Number of tanks and reservoirs
    LinkCount = EN_CountType_EN_LINKCOUNT, // Number of links (pipes + pumps + valves)
    PatternCount = EN_CountType_EN_PATCOUNT, // Number of time patterns
    CurveCount = EN_CountType_EN_CURVECOUNT, // Number of data curves
    ControlCount = EN_CountType_EN_CONTROLCOUNT, // Number of simple controls
    RuleCount = EN_CountType_EN_RULECOUNT, // Number of rule-based controls
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum ActionCodeType {
    Unconditional = EN_ActionCodeType_EN_UNCONDITIONAL, // Delete all controls and connecting links
    Conditional = EN_ActionCodeType_EN_CONDITIONAL, // Cancel object deletion if it appears in controls or has connecting links
}}
