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

    pub fn get_control(&self, index: i32) -> Result<Control> {
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

        let enabled = self.get_control_enabled(index)?;
        if result == 0 {
            Ok(Control {
                index,
                control_type: ControlType::from_i32(out_type).unwrap(),
                link_index: out_link_index,
                setting: out_setting,
                node_index: out_node_index,
                level: out_level,
                enabled,
            })
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn update_control(&self, control: &Control) -> Result<()> {
        let result = unsafe {
            ffi::EN_setcontrol(
                self.ph,
                control.index,
                control.control_type as i32,
                control.link_index,
                control.setting,
                control.node_index,
                control.level,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn delete_control(&self, control: Control) -> Result<()> {
        self.delete_control_by_index(control.index)
    }

    pub fn add_control(&self, control: Control) -> Result<()> {
        let mut out_index = 0;
        let result = unsafe {
            ffi::EN_addcontrol(
                self.ph,
                control.control_type as i32,
                control.link_index,
                control.setting,
                control.node_index,
                control.level,
                &mut out_index,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn delete_control_by_index(&self, index: i32) -> Result<()> {
        let result = unsafe { ffi::EN_deletecontrol(self.ph, index) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_control_enabled(&self, control_index: i32) -> Result<bool> {
        let mut out_enabled = 0;
        let result = unsafe { ffi::EN_getcontrolenabled(self.ph, control_index, &mut out_enabled) };
        if result == 0 {
            Ok(out_enabled != 0)
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_control_enabled(&self, control_index: i32, enabled: bool) -> Result<()> {
        let result = unsafe { ffi::EN_setcontrolenabled(self.ph, control_index, enabled as i32) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }
}
