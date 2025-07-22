#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)] // To quiet warnings about unused constants in the generate code.

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_char;
    use std::{
        ffi::{CStr, CString},
        i32,
    };

    static DATA_PATH_NET1: &str = "./EPANET/example-networks/Net1.inp";
    static DATA_PATH_RPT: &str = "";
    static DATA_PATH_OUT: &str = "";

    #[test]
    fn test_project() {
        let mut error;
        let mut ph: EN_Project = &mut Project { _unused: [] };

        unsafe {
            error = EN_createproject(&mut ph);
        }

        assert_eq!(error, 0);

        unsafe {
            error = EN_deleteproject(ph);
        }

        assert_eq!(error, 0);
    }

    #[test]
    fn test_nodes() {
        unsafe {
            let mut ph: EN_Project = &mut Project { _unused: [] };
            let error = EN_createproject(&mut ph);
            assert_eq!(error, 0);

            let netPtr = CString::new(DATA_PATH_NET1).unwrap();
            let pathPtr = CString::new(DATA_PATH_RPT).unwrap();
            let outPtr = CString::new(DATA_PATH_OUT).unwrap();
            let error = EN_open(ph, netPtr.as_ptr(), pathPtr.as_ptr(), outPtr.as_ptr());
            assert_eq!(error, 0);

            let node1 = "N1";
            let node2 = "N2";
            let node1CStr = CString::new(node1).unwrap();
            let node2CStr = CString::new(node2).unwrap();
            let mut index: i32 = 0;
            let error = EN_addnode(
                ph,
                node1CStr.as_ptr(),
                EN_NodeType_EN_JUNCTION as i32,
                &mut index as *mut i32,
            );
            assert_eq!(error, 0);
            let error = EN_addnode(
                ph,
                node2CStr.as_ptr(),
                EN_NodeType_EN_JUNCTION as i32,
                &mut index as *mut i32,
            );
            assert_eq!(error, 0);

            let error = EN_getnodeindex(ph, node1CStr.as_ptr(), &mut index as *mut i32);
            let mut out_id: Vec<c_char> = vec![0; EN_SizeLimits_EN_MAXMSG as usize];
            EN_getnodeid(ph, 10, out_id.as_mut_ptr());
            assert_eq!(error, 0);
            assert_eq!(index, 10);
            assert_eq!(CStr::from_ptr(out_id.as_ptr()).to_str().unwrap(), "N1");
            let error = EN_getnodeindex(ph, node2CStr.as_ptr(), &mut index as *mut i32);
            assert_eq!(error, 0);
            let error = EN_deletenode(ph, index, EN_ActionCodeType_EN_UNCONDITIONAL as i32);
            assert_eq!(error, 0);
        }
    }
}
