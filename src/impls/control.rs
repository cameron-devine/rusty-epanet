//! Simple Control-related API methods for EPANET.
//!
//! This module contains methods for getting and adding simple controls.
use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::control::{Control, ControlType};
use crate::EPANET;
use enum_primitive::FromPrimitive;

/// ## Simple Control APIs
impl EPANET {

    pub fn get_control(&self, index: i32) -> Result<Control<'_>> {
        let mut out_type = 0;
        let mut out_link_index = 0;
        let mut out_setting = 0.0;
        let mut out_node_index = 0;
        let mut out_level = 0.0;

        let result = unsafe {
            ffi::EN_getcontrol(
                self.ph,
                index,
                &mut out_type,
                &mut out_link_index,
                &mut out_setting,
                &mut out_node_index,
                &mut out_level,
            )
        };
        check_error(result)?;

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

    pub fn update_control(&self, control: &Control) -> Result<()> {
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

    pub fn get_control_enabled(&self, control_index: i32) -> Result<bool> {
        let mut out_enabled = 0;
        check_error(unsafe {
            ffi::EN_getcontrolenabled(self.ph, control_index, &mut out_enabled)
        })?;
        Ok(out_enabled != 0)
    }

    pub fn set_control_enabled(&self, control_index: i32, enabled: bool) -> Result<()> {
        check_error(unsafe {
            ffi::EN_setcontrolenabled(self.ph, control_index, enabled as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use rstest::*;

    #[rstest]
    fn test_add_update_delete_control(ph: EPANET) {
        // Obtain a valid link index to control
        let link_index = ph.get_link_index("10").unwrap();

        // Create a timer control for the given link
        let mut control = ph
            .add_control(ControlType::Timer, link_index, 0.0, 0, 3600.0, true)
            .unwrap();
        assert!(control.index() > 0);

        // Update the control's trigger level and disable it
        control.level = 7200.0;
        control.enabled = false;
        control.update().unwrap();

        let fetched = ph.get_control(control.index()).unwrap();
        assert_eq!(fetched.level, 7200.0);
        assert!(!fetched.enabled);

        // Finally delete the control
        control.delete().unwrap();
        assert!(ph.get_control(control.index()).is_err());
    }
}
