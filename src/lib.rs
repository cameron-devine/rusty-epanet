use bindings as ffi;
use std::ffi::CString;
use std::mem::MaybeUninit;

use epanet_error::*;

/// An EPANET Project wrapper
pub struct EPANET {
    ph: ffi::EN_Project,
}

impl EPANET {
    /// Reads an EPANET input file with no errors allowed.
    /// Pass the name of an existing EPANET-formatted input file,
    /// the name of a report file to be created (or “” if not needed), the name of a binary output file to be created (or “” if not needed).
    ///
    /// Will panic on failure.
    pub fn new(inp_path: &str, report_path: &str, out_path: &str) -> Result<Self> {
        // let mut ph: ffi::EN_Project = &mut ffi::Project::new();
        let mut ph: MaybeUninit<*mut ffi::Project> = MaybeUninit::<ffi::EN_Project>::uninit();
        match unsafe { ffi::EN_createproject(ph.as_mut_ptr()) } {
            0 => (),
            x => return Err(EPANETError::from(x)),
        };

        let inp: CString = CString::new(inp_path).unwrap();
        let rpt: CString = CString::new(report_path).unwrap();
        let out: CString = CString::new(out_path).unwrap();
        unsafe {
            match ffi::EN_open(ph.assume_init(), inp.as_ptr(), rpt.as_ptr(), out.as_ptr()) {
                0 => Ok(EPANET {
                    ph: ph.assume_init(),
                }),
                x => Err(EPANETError::from(x)),
            }
        }
    }
}

impl Drop for EPANET {
    fn drop(&mut self) {
        unsafe {
            ffi::EN_close(self.ph);
            ffi::EN_deleteproject(self.ph);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::NodeType;

    #[test]
    fn nodes() {
        let mut en_project: EPANET =
            EPANET::new("../libepanet-sys/EPANET/example-networks/Net1.inp", "", "")
                .expect("ERROR OPENING PROJECT");

        let index = EPANET::add_node(&mut en_project, "N2", NodeType::Junction).unwrap();
        assert_eq!(index, 10);
        let index = EPANET::get_node_index(&mut en_project, "N2").unwrap();
        assert_eq!(index, 10);
        let id = EPANET::get_node_id(&mut en_project, index).expect("Error getting the node id.");
        assert_eq!(id, "N2");
    }
}

mod bindings;
pub mod epanet_error;
pub mod impls;
pub mod types;
