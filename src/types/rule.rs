use crate::bindings::*;
use enum_primitive::*;

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum RuleObject {
    Node = EN_RuleObject_EN_R_NODE, // Clause refers to a node
    Link = EN_RuleObject_EN_R_LINK, // Clause refers to a link
    System = EN_RuleObject_EN_R_SYSTEM, // Clause refers to a system parameter (e.g., time)
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum RuleVariable {
    Demand = EN_RuleVariable_EN_R_DEMAND, // Nodal demand
    Head = EN_RuleVariable_EN_R_HEAD, // Nodal hydraulic head
    Grade = EN_RuleVariable_EN_R_GRADE, // Nodal hydraulic grade
    Level = EN_RuleVariable_EN_R_LEVEL, // Tank water level
    Pressure = EN_RuleVariable_EN_R_PRESSURE, // Nodal pressure
    Flow = EN_RuleVariable_EN_R_FLOW, // Link flow rate
    Status = EN_RuleVariable_EN_R_STATUS, // Link status
    Setting = EN_RuleVariable_EN_R_SETTING, // Link setting
    Power = EN_RuleVariable_EN_R_POWER, // Pump power output
    Time = EN_RuleVariable_EN_R_TIME, // Elapsed simulation time
    ClockTime = EN_RuleVariable_EN_R_CLOCKTIME, // Time of day
    FillTime = EN_RuleVariable_EN_R_FILLTIME, // Time to fill a tank
    DrainTime = EN_RuleVariable_EN_R_DRAINTIME, // Time to drain a tank
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum RuleOperator {
    Eq = EN_RuleOperator_EN_R_EQ, // Equal to
    Ne = EN_RuleOperator_EN_R_NE, // Not equal
    Le = EN_RuleOperator_EN_R_LE, // Less than or equal to
    Ge = EN_RuleOperator_EN_R_GE, // Greater than or equal to
    Lt = EN_RuleOperator_EN_R_LT, // Less than
    Gt = EN_RuleOperator_EN_R_GT, // Greater than
    Is = EN_RuleOperator_EN_R_IS, // Is equal to
    Not = EN_RuleOperator_EN_R_NOT, // Is not equal to
    Below = EN_RuleOperator_EN_R_BELOW, // Is below
    Above = EN_RuleOperator_EN_R_ABOVE, // Is above
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum RuleStatus {
    IsOpen = EN_RuleStatus_EN_R_IS_OPEN, // Link is open
    IsClosed = EN_RuleStatus_EN_R_IS_CLOSED, // Link is closed
    IsActive = EN_RuleStatus_EN_R_IS_ACTIVE, // Control valve is active
}}
enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum LogicalOperator {
    IF = 1,
    AND = 2,
    OR = 3
}}

/// Utility struct for rule based control information.
pub struct Rule {
    pub rule_id: String,
    pub premises: Vec<Premise>,
    pub then_actions: Vec<ActionClause>,
    pub else_actions: Option<Vec<ActionClause>>,
    pub priority: Option<u8>,
    pub enabled: bool,
}

pub struct Premise {
    pub logical_operator: LogicalOperator,
    pub rule_object: RuleObject,
    pub object_index: i32,
    pub variable: RuleVariable,
    pub rule_operator: RuleOperator,
    pub status: Option<RuleStatus>,
    pub value: f64,
}
pub struct ActionClause {
    pub link_index: i32,
    pub status: RuleStatus,
    pub setting: f64,
}