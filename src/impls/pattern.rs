//! Pattern APIS
//!
//! This module contains APIs for adding and fetching patterns in EPANET.
use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::MAX_ID_SIZE;
use crate::EPANET;
use std::path::Path;

/// ## Pattern APIs
impl EPANET {
    pub fn add_pattern(&self, id: &str) -> Result<()> {
        let c_id = std::ffi::CString::new(id).unwrap();
        check_error(unsafe { ffi::EN_addpattern(self.ph, c_id.as_ptr()) })
    }

    pub fn delete_pattern(&self, index: i32) -> Result<()> {
        check_error(unsafe { ffi::EN_deletepattern(self.ph, index) })
    }

    pub fn get_pattern_id(&self, index: i32) -> Result<String> {
        let mut out_id: Vec<std::ffi::c_char> = vec![0; MAX_ID_SIZE as usize + 1];
        check_error(unsafe { ffi::EN_getpatternid(self.ph, index, out_id.as_mut_ptr()) })?;
        let id = unsafe { std::ffi::CStr::from_ptr(out_id.as_ptr()) }
            .to_str()
            .unwrap_or("")
            .trim_end()
            .to_string();
        Ok(id)
    }

    pub fn set_pattern_id(&self, index: i32, id: &str) -> Result<()> {
        let c_id = std::ffi::CString::new(id).unwrap();
        check_error(unsafe { ffi::EN_setpatternid(self.ph, index, c_id.as_ptr()) })
    }

    pub fn get_pattern_length(&self, index: i32) -> Result<i32> {
        let mut out_length = 0;
        check_error(unsafe { ffi::EN_getpatternlen(self.ph, index, &mut out_length) })?;
        Ok(out_length)
    }

    pub fn get_pattern_value(&self, index: i32, period: i32) -> Result<f64> {
        let mut out_value = 0.0;
        check_error(unsafe { ffi::EN_getpatternvalue(self.ph, index, period, &mut out_value) })?;
        Ok(out_value)
    }

    pub fn set_pattern_value(&self, index: i32, period: i32, value: f64) -> Result<()> {
        check_error(unsafe { ffi::EN_setpatternvalue(self.ph, index, period, value) })
    }

    pub fn get_average_pattern_value(&self, index: i32) -> Result<f64> {
        let mut out_value = 0.0;
        check_error(unsafe { ffi::EN_getaveragepatternvalue(self.ph, index, &mut out_value) })?;
        Ok(out_value)
    }

    pub fn set_pattern(&self, index: i32, values: &[f64]) -> Result<()> {
        let c_values = values.as_ptr() as *mut f64;
        check_error(unsafe { ffi::EN_setpattern(self.ph, index, c_values, values.len() as i32) })
    }

    pub fn load_pattern_file(&self, file_name: &Path, id: &str) -> Result<()> {
        let c_file_name = std::ffi::CString::new(file_name.to_str().unwrap()).unwrap();
        let c_id = std::ffi::CString::new(id).unwrap();

        check_error(unsafe {
            ffi::EN_loadpatternfile(self.ph, c_file_name.as_ptr(), c_id.as_ptr())
        })
    }
}
