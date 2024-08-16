pub mod epanet_error;
use enum_primitive::*;
pub use epanet_error::EPANETError;
use ffi::{
    EN_ActionCodeType_EN_CONDITIONAL, EN_ActionCodeType_EN_UNCONDITIONAL,
    EN_NodeProperty_EN_BASEDEMAND, EN_NodeProperty_EN_CANOVERFLOW, EN_NodeProperty_EN_DEMAND,
    EN_NodeType_EN_JUNCTION, EN_NodeType_EN_RESERVOIR, EN_NodeType_EN_TANK,
    EN_SizeLimits_EN_MAXMSG,
};
use libepanet_sys as ffi;
use std::ffi::{c_char, CStr, CString};
use std::mem::MaybeUninit;

/// EPANET Result type with EPANET specific errors
type Result<T> = std::result::Result<T, EPANETError>;

/// An EPANET Project wrapper
pub struct EPANET {
    ph: ffi::EN_Project,
}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
/// Node types
pub enum ENNodeType {
    Junction = EN_NodeType_EN_JUNCTION,
    Reservoir = EN_NodeType_EN_RESERVOIR,
    Tank = EN_NodeType_EN_TANK,
}
}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
/// Node properties
pub enum ENNodeProperty {
    BaseDemand = EN_NodeProperty_EN_BASEDEMAND,
    CanOverFlow = EN_NodeProperty_EN_CANOVERFLOW,
    Demand = EN_NodeProperty_EN_DEMAND,
}
}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
/// Node properties
pub enum ENActionCode {
    Conditional = EN_ActionCodeType_EN_CONDITIONAL,
    Unconditional = EN_ActionCodeType_EN_UNCONDITIONAL,
}
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

    #[test]
    fn nodes() {
        let mut en_project: EPANET =
            EPANET::new("../libepanet-sys/EPANET/example-networks/Net1.inp", "", "")
                .expect("ERROR OPENING PROJECT");

        let index = EPANET::add_node(&mut en_project, "N2", ENNodeType::Junction).unwrap();
        assert_eq!(index, 10);
        let index = EPANET::get_node_index(&mut en_project, "N2").unwrap();
        assert_eq!(index, 10);
        let id = EPANET::get_node_id(&mut en_project, index).expect("Error getting the node id.");
        assert_eq!(id, "N2");
    }
}
