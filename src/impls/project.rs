//! Project-related API methods for EPANET.
//!
//! This module contains methods for getting or altering project information.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::{CountType, ObjectType, MAX_MSG_SIZE, MAX_TITLE_SIZE};
use crate::EPANET;
use std::cell::RefCell;
use std::ffi::{c_char, c_int, CStr, CString};
use std::mem::MaybeUninit;

/// ## Project APIs
impl EPANET {
    pub fn get_comment(&self, object_type: ObjectType, index: i32) -> Result<String> {
        let mut out_comment: Vec<c_char> = vec![0; MAX_MSG_SIZE as usize + 1usize];
        check_error(unsafe {
            ffi::EN_getcomment(self.ph, object_type as i32, index, out_comment.as_mut_ptr())
        })?;
        let comment = unsafe { CStr::from_ptr(out_comment.as_ptr()) }
            .to_str()
            .unwrap_or("")
            .trim_end()
            .to_string();
        Ok(comment)
    }

    pub fn set_comment(&self, object_type: ObjectType, index: i32, comment: &str) -> Result<()> {
        let _comment = CString::new(comment)?;
        check_error(unsafe { ffi::EN_setcomment(self.ph, object_type as i32, index, _comment.as_ptr()) })
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
    pub fn get_count(&self, count_type: CountType) -> Result<i32> {
        let mut count: MaybeUninit<c_int> = MaybeUninit::uninit();
        check_error(unsafe { ffi::EN_getcount(self.ph, count_type as i32, count.as_mut_ptr()) })?;
        Ok(unsafe { count.assume_init() })
    }

    pub fn get_title(&self) -> Result<String> {
        let mut out_line1: Vec<c_char> = vec![0; MAX_TITLE_SIZE as usize + 1usize];
        let mut out_line2: Vec<c_char> = vec![0; MAX_TITLE_SIZE as usize + 1usize];
        let mut out_line3: Vec<c_char> = vec![0; MAX_TITLE_SIZE as usize + 1usize];
        check_error(unsafe {
            ffi::EN_gettitle(
                self.ph,
                out_line1.as_mut_ptr(),
                out_line2.as_mut_ptr(),
                out_line3.as_mut_ptr(),
            )
        })?;
        let lines = [out_line1, out_line2, out_line3]
            .iter()
            .map(|buf| {
                let s = unsafe { CStr::from_ptr(buf.as_ptr()) };
                s.to_str().unwrap_or("").trim_end().to_string()
            })
            .collect::<Vec<_>>();
        let title = lines.join("\n");
        Ok(title)
    }

    pub fn get_tag(&self, object_type: ObjectType, index: i32) -> Result<String> {
        let mut out_tag: Vec<c_char> = vec![0; MAX_MSG_SIZE as usize + 1usize];
        check_error(unsafe { ffi::EN_gettag(self.ph, object_type as i32, index, out_tag.as_mut_ptr()) })?;
        let tag = unsafe { CStr::from_ptr(out_tag.as_ptr()) }
            .to_str()
            .unwrap_or("")
            .trim_end()
            .to_string();
        Ok(tag)
    }

    pub fn set_tag(&self, object_type: ObjectType, index: i32, tag: &str) -> Result<()> {
        let _tag = CString::new(tag)?;
        check_error(unsafe { ffi::EN_settag(self.ph, object_type as i32, index, _tag.as_ptr()) })
    }

    pub fn set_title(&self, title_line1: &str, title_line2: &str, title_line3: &str) -> Result<()> {
        let c_title1 = CString::new(title_line1).expect("Title contains null bytes");
        let c_title2 = CString::new(title_line2).expect("Title contains null bytes");
        let c_title3 = CString::new(title_line3).expect("Title contains null bytes");

        check_error(unsafe {
            ffi::EN_settitle(
                self.ph,
                c_title1.as_ptr(),
                c_title2.as_ptr(),
                c_title3.as_ptr(),
            )
        })
    }

    pub fn save_inp_file(&self, file_name: &str) -> Result<()> {
        let inp_file_c = CString::new(file_name).expect("inp_file contains null bytes");
        check_error(unsafe { ffi::EN_saveinpfile(self.ph, inp_file_c.as_ptr()) })
    }
}

/// Runs a complete EPANET simulation as a one-shot operation.
///
/// Creates its own project handle internally, runs the full open-solve-report-close
/// cycle via `EN_runproject`, and cleans up. This is a standalone function rather than
/// an `EPANET` method because `EN_runproject` closes the project internally — calling
/// it on a live `EPANET` instance would leave the handle in a closed state.
///
/// # Parameters
/// - `inp_file`: Path to the EPANET input file.
/// - `report_file`: Path for the output report file.
/// - `out_file`: Path for the binary output file (or empty string).
/// - `cb`: Optional C-style progress callback.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(EPANETError)` if the simulation fails.
pub fn run_project(
    inp_file: &str,
    report_file: &str,
    out_file: &str,
    cb: Option<unsafe extern "C" fn(*mut ::std::os::raw::c_char)>,
) -> Result<()> {
    let ph = EPANET::create_project_handle()?;

    let inp_file_c = CString::new(inp_file).expect("inp_file contains null bytes");
    let report_file_c = CString::new(report_file).expect("report_file contains null bytes");
    let out_file_c = CString::new(out_file).expect("out_file contains null bytes");

    let result = check_error(unsafe {
        ffi::EN_runproject(
            ph,
            inp_file_c.as_ptr(),
            report_file_c.as_ptr(),
            out_file_c.as_ptr(),
            cb,
        )
    });

    // EN_runproject calls EN_close internally, so only delete the handle.
    unsafe { ffi::EN_deleteproject(ph) };
    result
}

/// Runs a complete EPANET simulation with a safe Rust closure as progress callback.
///
/// This is a safe wrapper around [`run_project`] that accepts a regular Rust closure
/// instead of an `unsafe extern "C" fn`.
///
/// Creates its own project handle internally and cleans up after the simulation.
///
/// # Parameters
/// - `inp_file`: Path to the EPANET input file.
/// - `report_file`: Path for the output report file.
/// - `out_file`: Path for the binary output file (or empty string).
/// - `cb`: A closure that receives progress messages as `&str`.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(EPANETError)` if the simulation fails.
///
/// # Safety
/// Uses a thread-local trampoline internally because the EPANET C callback signature
/// (`void (*)(char*)`) does not support a user-data pointer. This means the callback
/// is not reentrant — calling this method concurrently on the same thread will
/// overwrite the stored closure. This is safe in practice because `EN_runproject`
/// is a blocking call.
pub fn run_project_with_callback<F: FnMut(&str)>(
    inp_file: &str,
    report_file: &str,
    out_file: &str,
    mut cb: F,
) -> Result<()> {
    thread_local! {
        static CALLBACK: RefCell<Option<*mut ()>> = RefCell::new(None);
    }

    unsafe extern "C" fn trampoline(msg: *mut c_char) {
        CALLBACK.with(|cell| {
            if let Some(ptr) = *cell.borrow() {
                let cb = &mut **(ptr as *mut *mut dyn FnMut(&str));
                let s = CStr::from_ptr(msg).to_string_lossy();
                cb(s.as_ref());
            }
        });
    }

    // SAFETY: We store a thin pointer (to a stack-local fat pointer) in the
    // thread-local. The pointer is only used during the synchronous
    // EN_runproject call below and cleared immediately after, so both
    // `cb_trait_ref` and `cb` are guaranteed to outlive their use.
    let mut cb_trait_ref: *mut dyn FnMut(&str) = &mut cb;
    let cb_ptr: *mut *mut dyn FnMut(&str) = &mut cb_trait_ref;
    CALLBACK.with(|cell| *cell.borrow_mut() = Some(cb_ptr as *mut ()));
    let result = run_project(inp_file, report_file, out_file, Some(trampoline));
    CALLBACK.with(|cell| *cell.borrow_mut() = None);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use crate::types::{CountType, ObjectType};
    use rstest::rstest;

    #[rstest]
    fn test_get_set_title(ph: EPANET) {
        let title = ph.get_title().unwrap();
        assert!(title.contains("EPANET Example Network 1"));

        ph.set_title("A", "B", "C").unwrap();
        assert_eq!(ph.get_title().unwrap(), "A\nB\nC");
    }

    #[rstest]
    fn test_get_set_comment(ph: EPANET) {
        ph.set_comment(ObjectType::Node, 1, "node comment").unwrap();
        assert_eq!(ph.get_comment(ObjectType::Node, 1).unwrap(), "node comment");

        ph.set_comment(ObjectType::Link, 1, "link comment").unwrap();
        assert_eq!(ph.get_comment(ObjectType::Link, 1).unwrap(), "link comment");
    }

    #[rstest]
    fn test_get_count(ph: EPANET) {
        assert_eq!(ph.get_count(CountType::NodeCount).unwrap(), 11);
        assert_eq!(ph.get_count(CountType::LinkCount).unwrap(), 13);
        assert_eq!(ph.get_count(CountType::PatternCount).unwrap(), 1);
        assert_eq!(ph.get_count(CountType::CurveCount).unwrap(), 1);
        assert_eq!(ph.get_count(CountType::ControlCount).unwrap(), 2);
        assert_eq!(ph.get_count(CountType::RuleCount).unwrap(), 0);
    }

    #[rstest]
    fn test_get_set_tag(ph: EPANET) {
        ph.set_tag(ObjectType::Node, 1, "mytag").unwrap();
        assert_eq!(ph.get_tag(ObjectType::Node, 1).unwrap(), "mytag");

        ph.set_tag(ObjectType::Link, 1, "linktag").unwrap();
        assert_eq!(ph.get_tag(ObjectType::Link, 1).unwrap(), "linktag");
    }

    #[rstest]
    fn test_save_inp_file(ph: EPANET) {
        let tmp_path = "test_save_output.inp";
        ph.save_inp_file(tmp_path).unwrap();

        let rpt = temp_rpt_path();
        let ph2 = EPANET::with_inp_file(tmp_path, &rpt, "").unwrap();
        assert_eq!(
            ph2.get_count(CountType::NodeCount).unwrap(),
            ph.get_count(CountType::NodeCount).unwrap()
        );

        let _ = std::fs::remove_file(tmp_path);
    }

    #[test]
    fn test_run_project() {
        let result = run_project(
            "src/impls/test_utils/net1.inp",
            "test_run.rpt",
            "",
            None,
        );
        assert!(result.is_ok());
        let _ = std::fs::remove_file("test_run.rpt");
    }

    #[test]
    fn test_run_project_with_callback() {
        let messages = std::cell::RefCell::new(Vec::new());
        let result = run_project_with_callback(
            "src/impls/test_utils/net1.inp",
            "test_run_cb.rpt",
            "",
            |msg| {
                messages.borrow_mut().push(msg.to_string());
            },
        );
        assert!(result.is_ok());
        assert!(!messages.borrow().is_empty());
        let _ = std::fs::remove_file("test_run_cb.rpt");
    }
}
