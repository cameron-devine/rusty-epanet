//! Simple Control-related API methods for EPANET.
//!
//! This module contains methods for getting and adding simple controls.
use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::control::{Control, ControlType};
use crate::EPANET;
use num_traits::FromPrimitive;

/// ## Simple Control APIs
impl EPANET {
    /// Retrieves a control by its index.
    ///
    /// Convenience method that calls [`get_control_by_index`](Self::get_control_by_index).
    pub fn get_control(&self, index: i32) -> Result<Control<'_>> {
        self.get_control_by_index(index)
    }

    /// Retrieves a control by its index.
    pub fn get_control_by_index(&self, index: i32) -> Result<Control<'_>> {
        let mut out_type = 0;
        let mut out_link_index = 0;
        let mut out_setting = 0.0;
        let mut out_node_index = 0;
        let mut out_level = 0.0;

        check_error(unsafe {
            ffi::EN_getcontrol(
                self.ph,
                index,
                &mut out_type,
                &mut out_link_index,
                &mut out_setting,
                &mut out_node_index,
                &mut out_level,
            )
        })?;

        let enabled = self.get_control_enabled(index)?;

        Ok(Control {
            project: self,
            index,
            control_type: ControlType::from_i32(out_type).unwrap(),
            link_index: out_link_index,
            setting: out_setting,
            node_index: out_node_index,
            level: out_level,
            enabled,
        })
    }

    // Helper methods - Internal API

    pub(crate) fn update_control(&self, control: &Control) -> Result<()> {
        check_error(unsafe {
            ffi::EN_setcontrol(
                self.ph,
                control.index,
                control.control_type as i32,
                control.link_index,
                control.setting,
                control.node_index,
                control.level,
            )
        })?;

        self.set_control_enabled(control.index, control.enabled)
    }

    pub fn delete_control(&self, control: Control) -> Result<()> {
        self.delete_control_by_index(control.index)
    }

    pub fn add_control(
        &self,
        control_type: ControlType,
        link_index: i32,
        setting: f64,
        node_index: i32,
        level: f64,
        enabled: bool,
    ) -> Result<Control<'_>> {
        let mut out_index = 0;
        check_error(unsafe {
            ffi::EN_addcontrol(
                self.ph,
                control_type as i32,
                link_index,
                setting,
                node_index,
                level,
                &mut out_index,
            )
        })?;

        self.set_control_enabled(out_index, enabled)?;
        Ok(Control {
            project: self,
            index: out_index,
            control_type,
            link_index,
            setting,
            node_index,
            level,
            enabled,
        })
    }

    fn delete_control_by_index(&self, index: i32) -> Result<()> {
        check_error(unsafe { ffi::EN_deletecontrol(self.ph, index) })
    }

    fn get_control_enabled(&self, control_index: i32) -> Result<bool> {
        let mut out_enabled = 0;
        check_error(unsafe {
            ffi::EN_getcontrolenabled(self.ph, control_index, &mut out_enabled)
        })?;
        Ok(out_enabled != 0)
    }

    fn set_control_enabled(&self, control_index: i32, enabled: bool) -> Result<()> {
        check_error(unsafe { ffi::EN_setcontrolenabled(self.ph, control_index, enabled as i32) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use crate::types::control::Control;
    use rstest::*;

    #[rstest]
    fn test_control_constructors(ph: EPANET) {
        let link_index = ph.get_link_index("10").unwrap();
        let tank_index = ph.get_node_index("2").unwrap();

        // Test low-level control
        let lowlevel = Control::new_lowlevel(&ph, link_index, 0.0, tank_index, 110.0).unwrap();
        assert_eq!(lowlevel.control_type, ControlType::LowLevel);
        assert_eq!(lowlevel.link_index, link_index);
        assert_eq!(lowlevel.setting, 0.0);
        assert_eq!(lowlevel.node_index, tank_index);
        assert_eq!(lowlevel.level, 110.0);
        assert!(lowlevel.enabled);

        // Test high-level control
        let hilevel = Control::new_hilevel(&ph, link_index, 1.0, tank_index, 140.0).unwrap();
        assert_eq!(hilevel.control_type, ControlType::HiLevel);
        assert_eq!(hilevel.setting, 1.0);
        assert_eq!(hilevel.level, 140.0);

        // Test timer control (node_index should be 0)
        let timer = Control::new_timer(&ph, link_index, 0.5, 3600.0).unwrap();
        assert_eq!(timer.control_type, ControlType::Timer);
        assert_eq!(timer.node_index, 0);
        assert_eq!(timer.level, 3600.0);
        assert_eq!(timer.setting, 0.5);

        // Test time-of-day control (node_index should be 0)
        let timeofday = Control::new_timeofday(&ph, link_index, 1.0, 28800.0).unwrap();
        assert_eq!(timeofday.control_type, ControlType::TimeOfDay);
        assert_eq!(timeofday.node_index, 0);
        assert_eq!(timeofday.level, 28800.0); // 8 AM = 8 * 3600 seconds
    }

    #[rstest]
    fn test_get_control_methods(ph: EPANET) {
        let link_index = ph.get_link_index("10").unwrap();

        // Create a timer control
        let created = Control::new_timer(&ph, link_index, 0.0, 3600.0).unwrap();
        let control_index = created.index();

        // Test get_control (convenience method)
        let control1 = ph.get_control(control_index).unwrap();
        assert_eq!(control1.control_type, ControlType::Timer);
        assert_eq!(control1.level, 3600.0);

        // Test get_control_by_index
        let control2 = ph.get_control_by_index(control_index).unwrap();
        assert_eq!(control2.index, control_index);
        assert_eq!(control2.link_index, link_index);
        assert_eq!(control2.setting, 0.0);
    }

    #[rstest]
    fn test_update_control(ph: EPANET) {
        let link_index = ph.get_link_index("10").unwrap();
        let tank_index = ph.get_node_index("2").unwrap();

        // Create a low-level control
        let mut control = Control::new_lowlevel(&ph, link_index, 0.0, tank_index, 110.0).unwrap();
        let control_index = control.index();

        // Update level
        control.level = 115.0;
        control.update().unwrap();
        let fetched = ph.get_control(control_index).unwrap();
        assert_eq!(fetched.level, 115.0);

        // Update setting
        control.setting = 1.0;
        control.update().unwrap();
        let fetched = ph.get_control(control_index).unwrap();
        assert_eq!(fetched.setting, 1.0);

        // Update enabled status
        control.enabled = false;
        control.update().unwrap();
        let fetched = ph.get_control(control_index).unwrap();
        assert!(!fetched.enabled);

        // Change control type to high-level
        control.control_type = ControlType::HiLevel;
        control.level = 140.0;
        control.update().unwrap();
        let fetched = ph.get_control(control_index).unwrap();
        assert_eq!(fetched.control_type, ControlType::HiLevel);
        assert_eq!(fetched.level, 140.0);
    }

    #[rstest]
    fn test_delete_control(ph: EPANET) {
        let link_index = ph.get_link_index("10").unwrap();

        // Create a timer control
        let control = Control::new_timer(&ph, link_index, 0.0, 3600.0).unwrap();
        let control_index = control.index();

        // Verify control exists
        assert!(ph.get_control(control_index).is_ok());

        // Delete control
        control.delete().unwrap();

        // Verify control no longer exists
        let result = ph.get_control(control_index);
        assert!(result.is_err());
    }
}
