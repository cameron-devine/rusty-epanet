use enum_primitive::*;
use crate::bindings as ffi;
use ffi::{
    EN_NodeProperty_EN_BASEDEMAND, EN_NodeProperty_EN_CANOVERFLOW, EN_NodeProperty_EN_DEMAND,
    EN_NodeType_EN_JUNCTION, EN_NodeType_EN_RESERVOIR, EN_NodeType_EN_TANK,
    EN_ActionCodeType_EN_CONDITIONAL, EN_ActionCodeType_EN_UNCONDITIONAL,

};
enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(i32)]
/// Node properties
pub enum ENActionCode {
    Conditional = EN_ActionCodeType_EN_CONDITIONAL,
    Unconditional = EN_ActionCodeType_EN_UNCONDITIONAL,
}
}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(i32)]
/// Node types
pub enum ENNodeType {
    Junction = EN_NodeType_EN_JUNCTION,
    Reservoir = EN_NodeType_EN_RESERVOIR,
    Tank = EN_NodeType_EN_TANK,
}
}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(i32)]
/// Node properties
pub enum ENNodeProperty {
    BaseDemand = EN_NodeProperty_EN_BASEDEMAND,
    CanOverFlow = EN_NodeProperty_EN_CANOVERFLOW,
    Demand = EN_NodeProperty_EN_DEMAND,
}
}