use crate::{
    bindings::{
        EN_ControlType_EN_HILEVEL, EN_ControlType_EN_LOWLEVEL, EN_ControlType_EN_TIMEOFDAY,
        EN_ControlType_EN_TIMER,
    },
    EPANET,
};
use enum_primitive::*;

/// A struct for holding simple control information.
///
/// `Control` instances hold a reference to their owning [`EPANET`] project so
/// that modifications can be synchronised back to the engine in an RAII
/// fashion. After mutating any of the public fields, call [`Control::update`]
/// to commit those changes. The control can also be removed from the model by
/// consuming it with [`Control::delete`].
#[derive(Debug, Clone)]
pub struct Control<'a> {
    /// Reference to the owning EPANET project.
    pub(crate) project: &'a EPANET,
    /// EPANET project index of the control.
    pub(crate) index: i32,
    /// The control type. (see [`ControlType`])
    pub control_type: ControlType,
    /// The index of the link to control starting from 1.
    pub link_index: i32,
    /// Control setting applied to the link.
    pub setting: f64,
    /// The index of the node used to control the link.
    /// 0 for [`ControlType::Timer`] and [`ControlType::TimeOfDay`].
    pub node_index: i32,
    /// action level (tank level, junction pressure, or time in seconds) that triggers the control.
    pub level: f64,
    /// Enabled status of the control.
    pub enabled: bool,
}

impl<'a> Control<'a> {
    /// Returns the EPANET project index of the control.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Synchronises any local changes of this control back to the EPANET engine.
    pub fn update(&self) -> crate::epanet_error::Result<()> {
        self.project.update_control(self)
    }

    /// Deletes this control from the EPANET project.
    pub fn delete(self) -> crate::epanet_error::Result<()> {
        self.project.delete_control(self)
    }
}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum ControlType {
    /// Act when pressure or tank level drops below a setpoint
    LowLevel = EN_ControlType_EN_LOWLEVEL,
    /// Act when pressure or tank level rises above a setpoint
    HiLevel = EN_ControlType_EN_HILEVEL,
    /// Act at a prescribed elapsed amount of time
    Timer = EN_ControlType_EN_TIMER,
    /// Act at a particular time of day
    TimeOfDay = EN_ControlType_EN_TIMEOFDAY,
}}
