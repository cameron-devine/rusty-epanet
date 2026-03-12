use crate::bindings::*;
use crate::EPANET;
use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum RuleObject {
    Node = EN_RuleObject_EN_R_NODE, // Clause refers to a node
    Link = EN_RuleObject_EN_R_LINK, // Clause refers to a link
    System = EN_RuleObject_EN_R_SYSTEM, // Clause refers to a system parameter (e.g., time)
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
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
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
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
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum RuleStatus {
    IsOpen = EN_RuleStatus_EN_R_IS_OPEN, // Link is open
    IsClosed = EN_RuleStatus_EN_R_IS_CLOSED, // Link is closed
    IsActive = EN_RuleStatus_EN_R_IS_ACTIVE, // Control valve is active
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum LogicalOperator {
    IF = 1,
    AND = 2,
    OR = 3
}

/// RAII struct for rule-based control information.
///
/// `Rule` instances hold a reference to their owning [`EPANET`] project so
/// that modifications can be synchronised back to the engine. After mutating
/// any of the public fields, call [`Rule::update`] to commit those changes.
/// The rule can also be removed from the model by consuming it with
/// [`Rule::delete`].
///
/// **Note:** The C API does not support adding or removing individual premises
/// or actions after rule creation. The `update()` method syncs the *existing*
/// elements in place. Pushing or popping from `premises`, `then_actions`, or
/// `else_actions` and calling `update()` will result in C API errors.
#[derive(Debug, Clone)]
pub struct Rule<'a> {
    /// Reference to the owning EPANET project.
    pub(crate) project: &'a EPANET,
    /// EPANET project index of the rule.
    pub(crate) index: i32,
    pub rule_id: String,
    pub premises: Vec<Premise>,
    pub then_actions: Vec<ActionClause>,
    pub else_actions: Vec<ActionClause>,
    pub priority: f64,
    pub enabled: bool,
}

impl<'a> Rule<'a> {
    /// Returns the EPANET project index of the rule.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Synchronises any local changes of this rule back to the EPANET engine.
    ///
    /// This pushes all premises, then-actions, else-actions, priority, and
    /// enabled status back to the C API. Only existing elements are updated;
    /// the C API does not support adding or removing premises/actions after
    /// rule creation.
    pub fn update(&self) -> crate::epanet_error::Result<()> {
        self.project.update_rule(self)
    }

    /// Deletes this rule from the EPANET project.
    ///
    /// This method consumes the rule, preventing further use after deletion.
    pub fn delete(self) -> crate::epanet_error::Result<()> {
        self.project.delete_rule(self.index)
    }
}

#[derive(Debug, Clone)]
pub struct Premise {
    pub logical_operator: LogicalOperator,
    pub rule_object: RuleObject,
    pub object_index: i32,
    pub variable: RuleVariable,
    pub rule_operator: RuleOperator,
    pub status: Option<RuleStatus>,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct ActionClause {
    pub link_index: i32,
    pub status: RuleStatus,
    pub setting: f64,
}
