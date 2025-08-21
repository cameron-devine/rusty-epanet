//! Demand API methods for EPANET.
//!
//! This module contains methods for setting and reading demand patterns.
use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::demand::*;
use crate::types::MAX_ID_SIZE;
use crate::EPANET;
use enum_primitive::FromPrimitive;
use std::ffi::{c_char, CString};

/// ## Demand APIs
impl EPANET {
    pub fn get_demand_model(&self) -> Result<DemandModelInfo> {
        let mut out_type = 0;
        let mut out_pmin = 0.0;
        let mut out_preq = 0.0;
        let mut out_pexp = 0.0;
        let result = unsafe {
            ffi::EN_getdemandmodel(
                self.ph,
                &mut out_type,
                &mut out_pmin,
                &mut out_preq,
                &mut out_pexp,
            )
        };
        if result == 0 {
            Ok(DemandModelInfo {
                demand_type: DemandModel::from_i32(out_type).unwrap(),
                pressure_min: out_pmin,
                pressure_required: out_preq,
                pressure_exponent: out_pexp,
            })
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_demand_model(&self, model: DemandModelInfo) -> Result<()> {
        let result = unsafe {
            ffi::EN_setdemandmodel(
                self.ph,
                model.demand_type as i32,
                model.pressure_min,
                model.pressure_required,
                model.pressure_exponent,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn add_demand(
        &self,
        node_index: i32,
        base_demand: f64,
        demand_pattern: &str,
        demand_name: &str,
    ) -> Result<()> {
        let c_demand_pattern = CString::new(demand_pattern).unwrap();
        let c_demand_name = CString::new(demand_name).unwrap();

        let result = unsafe {
            ffi::EN_adddemand(
                self.ph,
                node_index,
                base_demand,
                c_demand_pattern.as_ptr(),
                c_demand_name.as_ptr(),
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn delete_demand(&self, node_index: i32, demand_index: i32) -> Result<()> {
        let result = unsafe { ffi::EN_deletedemand(self.ph, node_index, demand_index) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_demand_index(&self, node_index: i32, demand_name: &str) -> Result<i32> {
        let mut out_index = 0;
        let c_demand_name = CString::new(demand_name).unwrap();

        let result = unsafe {
            ffi::EN_getdemandindex(self.ph, node_index, c_demand_name.as_ptr(), &mut out_index)
        };
        if result == 0 {
            Ok(out_index)
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_demand_count(&self, node_index: i32) -> Result<i32> {
        let mut out_count = 0;
        let result = unsafe { ffi::EN_getnumdemands(self.ph, node_index, &mut out_count) };
        if result == 0 {
            Ok(out_count)
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_base_demand(&self, node_index: i32, demand_index: i32) -> Result<f64> {
        let mut out_demand = 0.0;
        let result =
            unsafe { ffi::EN_getbasedemand(self.ph, node_index, demand_index, &mut out_demand) };
        if result == 0 {
            Ok(out_demand)
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_base_demand(&self, node_index: i32, demand_index: i32, demand: f64) -> Result<()> {
        let result = unsafe { ffi::EN_setbasedemand(self.ph, node_index, demand_index, demand) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_demand_pattern(&self, node_index: i32, demand_index: i32) -> Result<i32> {
        let mut out_pattern = 0;
        let result = unsafe {
            ffi::EN_getdemandpattern(self.ph, node_index, demand_index, &mut out_pattern)
        };
        if result == 0 {
            Ok(out_pattern)
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_demand_pattern(
        &self,
        node_index: i32,
        demand_index: i32,
        pattern_index: i32,
    ) -> Result<()> {
        let result =
            unsafe { ffi::EN_setdemandpattern(self.ph, node_index, demand_index, pattern_index) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_demand_name(&self, node_index: i32, demand_index: i32) -> Result<String> {
        let mut out_name: Vec<c_char> = vec![0; MAX_ID_SIZE as usize + 1usize];
        let result = unsafe {
            ffi::EN_getdemandname(self.ph, node_index, demand_index, out_name.as_mut_ptr())
        };
        if result == 0 {
            let name = unsafe { std::ffi::CStr::from_ptr(out_name.as_ptr()) }
                .to_string_lossy()
                .trim_end()
                .to_string();
            Ok(name)
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_demand_name(&self, node_index: i32, demand_index: i32, name: &str) -> Result<()> {
        let c_name = CString::new(name).expect("Demand name contains null bytes");
        let result =
            unsafe { ffi::EN_setdemandname(self.ph, node_index, demand_index, c_name.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use rstest::rstest;
    use std::fs;

    #[rstest]
    pub fn test_demands(ph: EPANET) {
        let mut result = ph.get_node_index("12");
        assert!(result.is_ok());

        let node_index = result.unwrap();
        result = ph.get_demand_count(node_index);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        let name_result = ph.get_demand_name(node_index, 1);
        assert!(name_result.is_ok());

        let set_result = ph.set_demand_name(node_index, 1, "CUB_SCOUT_MOTOR_POOL");
        assert!(set_result.is_ok());
        let save_result = ph.save_inp_file("net1_dem_cat.inp");
        assert!(save_result.is_ok());

        drop(ph);

        let new_ph = EPANET::with_inp_file("net1_dem_cat.inp", "", "").unwrap();

        let new_node_index_result = new_ph.get_node_index("12");
        assert!(new_node_index_result.is_ok());
        let new_node_index = new_node_index_result.unwrap();
        let new_demand_count_result = new_ph.get_demand_count(new_node_index);
        assert!(new_demand_count_result.is_ok());
        assert_eq!(new_demand_count_result.unwrap(), 1);

        let new_demand_name_result = new_ph.get_demand_name(new_node_index, 1);
        assert!(new_demand_name_result.is_ok());
        let new_demand_name = new_demand_name_result.unwrap();
        assert_eq!(new_demand_name, "CUB_SCOUT_MOTOR_POOL");

        fs::remove_file("net1_dem_cat.inp").expect("Failed to remove file");
    }

    #[rstest]
    pub fn test_add_demand(ph_single_node: (EPANET, i32)) {
        let (ph, node_qhut) = ph_single_node;

        let mut result = ph.add_demand(node_qhut, 100.0, "PrimaryPattern", "PrimaryDemand");
        println!("{:?}", result);
        assert!(result.is_err());

        result = ph.add_pattern("PrimaryPattern");
        assert!(result.is_ok());

        result = ph.add_demand(node_qhut, 100.0, "PrimaryPattern", "PrimaryDemand");
        assert!(result.is_ok());

        result = ph.add_pattern("SecondaryPattern");
        assert!(result.is_ok());

        result = ph.add_demand(node_qhut, 10.0, "SecondaryPattern", "SecondaryDemand");
        assert!(result.is_ok());

        result = ph.add_pattern("TertiaryPattern");
        assert!(result.is_ok());

        result = ph.add_demand(node_qhut, 1.0, "TertiaryPattern", "TertiaryDemand");
        assert!(result.is_ok());

        let count_result = ph.get_demand_count(node_qhut);
        assert!(count_result.is_ok());
        let count = count_result.unwrap();

        let index_result = ph.get_demand_index(node_qhut, "TertiaryDemand");
        assert!(index_result.is_ok());
        let index = index_result.unwrap();
        assert_eq!(index, count);

        result = ph.delete_demand(node_qhut, index);
        assert!(result.is_ok());

        let count2_result = ph.get_demand_count(node_qhut);
        assert!(count2_result.is_ok());
        assert_eq!(count2_result.unwrap(), count - 1);
    }
}
