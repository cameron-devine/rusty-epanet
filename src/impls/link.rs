//! Link-related API methods for EPANET.
//!
//! This module contains methods for adding, deleting, and querying links.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::link::*;
use crate::types::MAX_ID_SIZE;
use crate::types::{ActionCodeType, CountType::LinkCount};
use crate::EPANET;
use enum_primitive::FromPrimitive;
use std::ffi::{c_char, CString};

/// ## Link APIs
impl EPANET {
    pub fn delete_link(&self, index: i32, action_code_type: ActionCodeType) -> Result<()> {
        check_error(unsafe { ffi::EN_deletelink(self.ph, index, action_code_type as i32) })
    }

    pub fn get_link_index(&self, id: &str) -> Result<i32> {
        let c_id = CString::new(id).unwrap();
        let mut out_index = 0;
        check_error(unsafe { ffi::EN_getlinkindex(self.ph, c_id.as_ptr(), &mut out_index) })?;
        Ok(out_index)
    }

    pub fn get_link_id(&self, index: i32) -> Result<String> {
        let mut out_id: Vec<c_char> = vec![0; MAX_ID_SIZE as usize + 1usize];
        check_error(unsafe { ffi::EN_getlinkid(self.ph, index, out_id.as_mut_ptr()) })?;
        let id = unsafe { std::ffi::CStr::from_ptr(out_id.as_ptr()) }
            .to_string_lossy()
            .trim_end()
            .to_string();
        Ok(id)
    }

    pub fn set_link_id(&self, index: i32, id: &str) -> Result<()> {
        let c_id = CString::new(id).unwrap();
        check_error(unsafe { ffi::EN_setlinkid(self.ph, index, c_id.as_ptr()) })
    }

    pub fn get_link_type(&self, index: i32) -> Result<LinkType> {
        let mut out_type = 0;
        check_error(unsafe { ffi::EN_getlinktype(self.ph, index, &mut out_type) })?;
        Ok(LinkType::from_i32(out_type).unwrap())
    }

    pub fn set_link_type(
        &self,
        index: i32,
        link_type: LinkType,
        action_code: ActionCodeType,
    ) -> Result<i32> {
        let mut in_out_index = index;
        let result = unsafe {
            ffi::EN_setlinktype(
                self.ph,
                &mut in_out_index,
                link_type as i32,
                action_code as i32,
            )
        };
        check_error(result)?;
        Ok(in_out_index)
    }

    pub fn get_link_nodes(&self, index: i32) -> Result<(i32, i32)> {
        let (mut out_node1, mut out_node2) = (0, 0);
        check_error(unsafe {
            ffi::EN_getlinknodes(self.ph, index, &mut out_node1, &mut out_node2)
        })?;
        Ok((out_node1, out_node2))
    }

    pub fn set_link_nodes(&self, index: i32, node1: i32, node2: i32) -> Result<()> {
        check_error(unsafe { ffi::EN_setlinknodes(self.ph, index, node1, node2) })
    }

    pub fn get_link_value(&self, index: i32, property: LinkProperty) -> Result<f64> {
        let mut out_value = 0.0;
        check_error(unsafe {
            ffi::EN_getlinkvalue(self.ph, index, property as i32, &mut out_value)
        })?;
        Ok(out_value)
    }

    pub fn get_link_values(&self, property: LinkProperty) -> Result<Vec<f64>> {
        let link_count = match self.get_count(LinkCount) {
            Ok(count) => count,
            Err(e) => return Err(e),
        };
        let mut values: Vec<f64> = vec![0.0; link_count as usize];
        let result =
            unsafe { ffi::EN_getlinkvalues(self.ph, property as i32, values.as_mut_ptr()) };
        if result == 0 {
            Ok(values)
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_link_value(&self, index: i32, property: LinkProperty, value: f64) -> Result<()> {
        let result = unsafe { ffi::EN_setlinkvalue(self.ph, index, property as i32, value) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_pipe_data(
        &self,
        index: i32,
        length: f64,
        diameter: f64,
        roughness: f64,
        minor_loss: f64,
    ) -> Result<()> {
        let result =
            unsafe { ffi::EN_setpipedata(self.ph, index, length, diameter, roughness, minor_loss) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_pump_type(&self, index: i32) -> Result<PumpType> {
        let mut out_type = 0;
        check_error(unsafe { ffi::EN_getpumptype(self.ph, index, &mut out_type) })?;
        Ok(PumpType::from_i32(out_type).unwrap())
    }

    pub fn get_head_curve_index(&self, link_index: i32) -> Result<i32> {
        let mut out_index = 0;
        check_error(unsafe { ffi::EN_getheadcurveindex(self.ph, link_index, &mut out_index) })?;
        Ok(out_index)
    }

    pub fn set_head_curve_index(&self, link_index: i32, curve_index: i32) -> Result<()> {
        check_error(unsafe { ffi::EN_setheadcurveindex(self.ph, link_index, curve_index) })
    }

    pub fn get_vertex_count(&self, link_index: i32) -> Result<i32> {
        let mut out_count = 0;
        check_error(unsafe { ffi::EN_getvertexcount(self.ph, link_index, &mut out_count) })?;
        Ok(out_count)
    }

    pub fn get_vertex(&self, link_index: i32, vertex_index: i32) -> Result<(f64, f64)> {
        let (mut out_x, mut out_y) = (0.0, 0.0);
        check_error(unsafe {
            ffi::EN_getvertex(self.ph, link_index, vertex_index, &mut out_x, &mut out_y)
        })?;
        Ok((out_x, out_y))
    }

    pub fn set_vertex(&self, link_index: i32, vertex_index: i32, x: f64, y: f64) -> Result<()> {
        check_error(unsafe { ffi::EN_setvertex(self.ph, link_index, vertex_index, x, y) })
    }

    pub fn set_vertices(&self, link_index: i32, vertices: Vec<(f64, f64)>) -> Result<()> {
        let (mut xs, mut ys): (Vec<f64>, Vec<f64>) = vertices.iter().cloned().unzip();
        check_error(unsafe {
            ffi::EN_setvertices(
                self.ph,
                link_index,
                xs.as_mut_ptr(),
                ys.as_mut_ptr(),
                vertices.len() as i32,
            )
        })
    }
}
