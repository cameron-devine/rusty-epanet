use crate::EPANET;
use crate::bindings as ffi;
use ffi::{
    EN_SizeLimits_EN_MAXMSG,
};
use crate::types as types;
use types::{ENNodeType, ENActionCode};
use crate::epanet_error::*;

use std::ffi::{c_char, CStr, CString};
use std::mem::MaybeUninit;

impl EPANET {
    /// Add a node to the project with a given name and type.
    /// Returns the index of the node or an error.
    pub fn add_node(&mut self, id: &str, node_type: ENNodeType) -> Result<i32> {
        let _id = CString::new(id).unwrap();
        let mut out_index = MaybeUninit::uninit();
        unsafe {
            match ffi::EN_addnode(
                self.ph,
                _id.as_ptr(),
                node_type as i32,
                out_index.as_mut_ptr(),
            ) {
                0 => Ok(out_index.assume_init()),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    pub fn delete_node(&mut self, id: i32, action_code: ENActionCode) -> Result<()> {
        unsafe {
            match ffi::EN_deletenode(self.ph, id, action_code as i32) {
                0 => Ok(()),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Get the index of the node with the given id
    pub fn get_node_index(&mut self, id: &str) -> Result<i32> {
        let _id = CString::new(id).unwrap();
        let mut out_index = MaybeUninit::uninit();
        unsafe {
            match ffi::EN_getnodeindex(self.ph, _id.as_ptr(), out_index.as_mut_ptr()) {
                0 => Ok(out_index.assume_init()),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Get a node id given an index
    pub fn get_node_id(&mut self, index: i32) -> Result<String> {
        let mut out_id: Vec<c_char> = vec![0; EN_SizeLimits_EN_MAXMSG as usize];
        unsafe {
            match ffi::EN_getnodeid(self.ph, index, out_id.as_mut_ptr()) {
                0 => Ok(CStr::from_ptr(out_id.as_ptr())
                    .to_str()
                    .unwrap()
                    .to_string()),
                x => Err(EPANETError::from(x)),
            }
        }
    }
}