use crate::{
    bindings::{
        EN_ControlType_EN_HILEVEL, EN_ControlType_EN_LOWLEVEL, EN_ControlType_EN_TIMEOFDAY,
        EN_ControlType_EN_TIMER,
    },
    EPANET,
};
use num_derive::FromPrimitive;

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
    /// Creates a new low-level control that acts when pressure or tank level drops below a setpoint.
    ///
    /// # Parameters
    /// - `project`: Reference to the EPANET project
    /// - `link_index`: Index of the link to control
    /// - `setting`: Control setting applied to the link (0.0 for closed, 1.0 for open, or speed multiplier for pumps)
    /// - `node_index`: Index of the node (tank or junction) whose level/pressure triggers the control
    /// - `level`: Level or pressure setpoint that triggers the control
    pub fn new_lowlevel(
        project: &'a EPANET,
        link_index: i32,
        setting: f64,
        node_index: i32,
        level: f64,
    ) -> crate::epanet_error::Result<Self> {
        project.add_control(ControlType::LowLevel, link_index, setting, node_index, level, true)
    }

    /// Creates a new high-level control that acts when pressure or tank level rises above a setpoint.
    ///
    /// # Parameters
    /// - `project`: Reference to the EPANET project
    /// - `link_index`: Index of the link to control
    /// - `setting`: Control setting applied to the link (0.0 for closed, 1.0 for open, or speed multiplier for pumps)
    /// - `node_index`: Index of the node (tank or junction) whose level/pressure triggers the control
    /// - `level`: Level or pressure setpoint that triggers the control
    pub fn new_hilevel(
        project: &'a EPANET,
        link_index: i32,
        setting: f64,
        node_index: i32,
        level: f64,
    ) -> crate::epanet_error::Result<Self> {
        project.add_control(ControlType::HiLevel, link_index, setting, node_index, level, true)
    }

    /// Creates a new timer control that acts at a prescribed elapsed amount of time.
    ///
    /// # Parameters
    /// - `project`: Reference to the EPANET project
    /// - `link_index`: Index of the link to control
    /// - `setting`: Control setting applied to the link (0.0 for closed, 1.0 for open, or speed multiplier for pumps)
    /// - `time`: Elapsed time in seconds when the control activates
    pub fn new_timer(
        project: &'a EPANET,
        link_index: i32,
        setting: f64,
        time: f64,
    ) -> crate::epanet_error::Result<Self> {
        project.add_control(ControlType::Timer, link_index, setting, 0, time, true)
    }

    /// Creates a new time-of-day control that acts at a particular time of day.
    ///
    /// # Parameters
    /// - `project`: Reference to the EPANET project
    /// - `link_index`: Index of the link to control
    /// - `setting`: Control setting applied to the link (0.0 for closed, 1.0 for open, or speed multiplier for pumps)
    /// - `time_of_day`: Time of day in seconds since midnight when the control activates
    pub fn new_timeofday(
        project: &'a EPANET,
        link_index: i32,
        setting: f64,
        time_of_day: f64,
    ) -> crate::epanet_error::Result<Self> {
        project.add_control(ControlType::TimeOfDay, link_index, setting, 0, time_of_day, true)
    }

    /// Returns the EPANET project index of the control.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Synchronises any local changes of this control back to the EPANET engine.
    pub fn update(&self) -> crate::epanet_error::Result<()> {
        self.project.update_control(self)
    }

    /// Deletes this control from the EPANET project.
    ///
    /// This method consumes the control, preventing further use after deletion.
    pub fn delete(self) -> crate::epanet_error::Result<()> {
        self.project.delete_control(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum ControlType {
    /// Act when pressure or tank level drops below a setpoint
    LowLevel = EN_ControlType_EN_LOWLEVEL as i32,
    /// Act when pressure or tank level rises above a setpoint
    HiLevel = EN_ControlType_EN_HILEVEL as i32,
    /// Act at a prescribed elapsed amount of time
    Timer = EN_ControlType_EN_TIMER as i32,
    /// Act at a particular time of day
    TimeOfDay = EN_ControlType_EN_TIMEOFDAY as i32,
}
