//! Project-related API methods for EPANET.
//!
//! This module contains methods for getting or altering project information.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::{CountType, FlowUnits, HeadLossType, ObjectType, MAX_MSG_SIZE, MAX_TITLE_SIZE};
use crate::EPANET;
use std::ffi::{c_char, c_int, CStr, CString};
use std::mem::MaybeUninit;
use std::ptr::null_mut;

/// ## Project APIs
impl EPANET {

    pub fn get_comment(&mut self, object_type: ObjectType, index: i32) -> Result<String> {
        let mut out_comment: Vec<c_char> = vec![0; MAX_MSG_SIZE as usize + 1usize];
        let result = unsafe { ffi::EN_getcomment(self.ph, object_type as i32, index, out_comment.as_mut_ptr()) };
        if result == 0 {
            let comment = unsafe { CStr::from_ptr(out_comment.as_ptr()) }
                .to_str()
                .unwrap_or("")
                .trim_end()
                .to_string();
            Ok(comment)
        } else {
            Err(EPANETError::from(result))
        }
    }
    /// Returns the number of objects of a specified type in the current EPANET project.
    ///
    /// # Parameters
    /// - `count_type`: The type of object to count, represented by the [`CountType`] enum.
    ///
    /// # Returns
    /// - `Ok(i32)`: The number of objects of the given type.
    /// - `Err(EPANETError)`: If the underlying EPANET API call fails.
    ///
    /// # Errors
    /// Return an error if the EPANET C API function `EN_getcount` fails.
    ///
    /// # See Also
    /// - EN_getcount (EPANET C API)
    /// - [`CountType`] for possible node types.
    pub fn get_count(&mut self, count_type: CountType) -> Result<i32> {
        let mut count: MaybeUninit<c_int> = MaybeUninit::uninit();
        let result = unsafe { ffi::EN_getcount(self.ph, count_type as i32, count.as_mut_ptr()) };
        if result == 0 {
            Ok(unsafe { count.assume_init() })
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_title(&mut self) -> Result<String> {
        let mut out_line1: Vec<c_char> = vec![0; MAX_TITLE_SIZE as usize + 1usize];
        let mut out_line2: Vec<c_char> = vec![0; MAX_TITLE_SIZE as usize + 1usize];
        let mut out_line3: Vec<c_char> = vec![0; MAX_TITLE_SIZE as usize + 1usize];
        let result = unsafe {
            ffi::EN_gettitle(
                self.ph,
                out_line1.as_mut_ptr(),
                out_line2.as_mut_ptr(),
                out_line3.as_mut_ptr(),
            )
        };
        if result == 0 {
            let lines = [out_line1, out_line2, out_line3]
                .iter()
                .map(|buf| {
                    let s = unsafe { CStr::from_ptr(buf.as_ptr()) };
                    s.to_str().unwrap_or("").trim_end().to_string()
                })
                .collect::<Vec<_>>();
            let title = lines.join("\n");
            Ok(title)
        } else {
            Err(EPANETError::from(result))
        }
    }

    // todo: figure out why EN_gettag is not in the bindings
    /*
    pub fn get_tag(&mut self, object_type: ObjectType, index: i32) -> Result<String> {
        let mut out_tag: Vec<c_char> = vec![0; MAX_MSG_SIZE as usize + 1usize];
        let result = unsafe { ffi::EN_gettag(self.ph, object_type as i32, index, out_tag.as_mut_ptr()) };
        if result == 0 {
            let tag = unsafe { CStr::from_ptr(out_tag.as_ptr()) }
                .to_str()
                .unwrap_or("")
                .trim_end()
                .to_string();
            Ok(tag)
        } else {
            Err(EPANETError::from(result))
        }
    }
    */

    pub fn set_title(
        &mut self,
        title_line1: &str,
        title_line2: &str,
        title_line3: &str,
    ) -> Result<()> {
        let c_title1 = CString::new(title_line1).expect("Title contains null bytes");
        let c_title2 = CString::new(title_line2).expect("Title contains null bytes");
        let c_title3 = CString::new(title_line3).expect("Title contains null bytes");

        match unsafe {
            ffi::EN_settitle(
                self.ph,
                c_title1.as_ptr(),
                c_title2.as_ptr(),
                c_title3.as_ptr(),
            )
        } {
            0 => Ok(()),
            x => Err(EPANETError::from(x)),
        }
    }

    pub fn run_project(&mut self, inp_file: &str, report_file: &str, out_file: &str, cb: Option<unsafe extern "C" fn(*mut ::std::os::raw::c_char)>) -> Result<()> {
        let inp_file_c = CString::new(inp_file).expect("inp_file contains null bytes");
        let report_file_c = CString::new(report_file).expect("report_file contains null bytes");
        let out_file_c = CString::new(out_file).expect("out_file contains null bytes");
        let result = unsafe { ffi::EN_runproject(self.ph, inp_file_c.as_ptr(), report_file_c.as_ptr(), out_file_c.as_ptr(), cb) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn save_inp_file(&mut self, file_name: &str) -> Result<()> {
        let inp_file_c = CString::new(file_name).expect("inp_file contains null bytes");
        let result = unsafe { ffi::EN_saveinpfile(self.ph, inp_file_c.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }
}
