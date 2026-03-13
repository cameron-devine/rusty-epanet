//! Demand API methods for EPANET.
//!
//! This module contains low-level APIs for managing demand categories on nodes,
//! as well as higher-level constructors that return [`Demand`] structs.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::demand::*;
use crate::types::MAX_ID_SIZE;
use crate::EPANET;
use num_traits::FromPrimitive;
use std::ffi::{c_char, CString};

/// ## Demand APIs
impl EPANET {
    /// Creates a new demand category on a node and returns a [`Demand`] struct.
    ///
    /// # Parameters
    ///
    /// - `node_index`: 1-based index of the node
    /// - `base_demand`: Base demand value
    /// - `demand_pattern`: ID of the time pattern (empty string for no pattern)
    /// - `name`: Name for the demand category
    ///
    /// # Returns
    ///
    /// A [`Demand`] struct with the cached fields populated.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the demand cannot be created.
    pub fn create_demand(
        &self,
        node_index: i32,
        base_demand: f64,
        demand_pattern: &str,
        name: &str,
    ) -> Result<Demand<'_>> {
        self.add_demand(node_index, base_demand, demand_pattern, name)?;
        let demand_index = self.get_demand_count(node_index)?;
        let pattern_index = self.get_demand_pattern(node_index, demand_index)?;

        Ok(Demand {
            project: self,
            node_index,
            demand_index,
            base_demand,
            pattern_index,
            name: name.to_string(),
        })
    }

    /// Retrieves a demand category by its name on a node.
    ///
    /// # Parameters
    ///
    /// - `node_index`: 1-based index of the node
    /// - `name`: Name of the demand category
    ///
    /// # Returns
    ///
    /// A [`Demand`] struct with all fields populated from the C engine.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the demand does not exist.
    pub fn get_demand(&self, node_index: i32, name: &str) -> Result<Demand<'_>> {
        let demand_index = self.get_demand_index(node_index, name)?;
        self.get_demand_by_index(node_index, demand_index)
    }

    /// Retrieves a demand category by its 1-based index on a node.
    ///
    /// # Parameters
    ///
    /// - `node_index`: 1-based index of the node
    /// - `demand_index`: 1-based index of the demand category on the node
    ///
    /// # Returns
    ///
    /// A [`Demand`] struct with all fields populated from the C engine.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the indices are invalid.
    pub fn get_demand_by_index(&self, node_index: i32, demand_index: i32) -> Result<Demand<'_>> {
        let base_demand = self.get_base_demand(node_index, demand_index)?;
        let pattern_index = self.get_demand_pattern(node_index, demand_index)?;
        let name = self.get_demand_name(node_index, demand_index)?;

        Ok(Demand {
            project: self,
            node_index,
            demand_index,
            base_demand,
            pattern_index,
            name,
        })
    }

    // =========================================================================
    // Low-level FFI wrappers
    // =========================================================================

    /// Returns the demand model parameters for the project.
    pub fn get_demand_model(&self) -> Result<DemandModelInfo> {
        let mut out_type = 0;
        let mut out_pmin = 0.0;
        let mut out_preq = 0.0;
        let mut out_pexp = 0.0;
        check_error(unsafe {
            ffi::EN_getdemandmodel(
                self.ph,
                &mut out_type,
                &mut out_pmin,
                &mut out_preq,
                &mut out_pexp,
            )
        })?;
        Ok(DemandModelInfo {
            demand_type: DemandModel::from_i32(out_type).unwrap(),
            pressure_min: out_pmin,
            pressure_required: out_preq,
            pressure_exponent: out_pexp,
        })
    }

    /// Sets the demand model parameters for the project.
    pub fn set_demand_model(&self, model: DemandModelInfo) -> Result<()> {
        check_error(unsafe {
            ffi::EN_setdemandmodel(
                self.ph,
                model.demand_type as i32,
                model.pressure_min,
                model.pressure_required,
                model.pressure_exponent,
            )
        })
    }

    /// Adds a new demand category to a node.
    pub fn add_demand(
        &self,
        node_index: i32,
        base_demand: f64,
        demand_pattern: &str,
        demand_name: &str,
    ) -> Result<()> {
        let c_demand_pattern = CString::new(demand_pattern).unwrap();
        let c_demand_name = CString::new(demand_name).unwrap();

        check_error(unsafe {
            ffi::EN_adddemand(
                self.ph,
                node_index,
                base_demand,
                c_demand_pattern.as_ptr(),
                c_demand_name.as_ptr(),
            )
        })
    }

    /// Deletes a demand category from a node.
    pub fn delete_demand(&self, node_index: i32, demand_index: i32) -> Result<()> {
        check_error(unsafe { ffi::EN_deletedemand(self.ph, node_index, demand_index) })
    }

    /// Returns the 1-based index of a demand category given its name on a node.
    pub fn get_demand_index(&self, node_index: i32, demand_name: &str) -> Result<i32> {
        let mut out_index = 0;
        let c_demand_name = CString::new(demand_name).unwrap();

        check_error(unsafe {
            ffi::EN_getdemandindex(self.ph, node_index, c_demand_name.as_ptr(), &mut out_index)
        })?;
        Ok(out_index)
    }

    /// Returns the number of demand categories on a node.
    pub fn get_demand_count(&self, node_index: i32) -> Result<i32> {
        let mut out_count = 0;
        check_error(unsafe { ffi::EN_getnumdemands(self.ph, node_index, &mut out_count) })?;
        Ok(out_count)
    }

    /// Returns the base demand value of a demand category on a node.
    pub fn get_base_demand(&self, node_index: i32, demand_index: i32) -> Result<f64> {
        let mut out_demand = 0.0;
        check_error(unsafe {
            ffi::EN_getbasedemand(self.ph, node_index, demand_index, &mut out_demand)
        })?;
        Ok(out_demand)
    }

    /// Sets the base demand value of a demand category on a node.
    pub fn set_base_demand(&self, node_index: i32, demand_index: i32, demand: f64) -> Result<()> {
        check_error(unsafe { ffi::EN_setbasedemand(self.ph, node_index, demand_index, demand) })
    }

    /// Returns the pattern index for a demand category on a node (0 = no pattern).
    pub fn get_demand_pattern(&self, node_index: i32, demand_index: i32) -> Result<i32> {
        let mut out_pattern = 0;
        check_error(unsafe {
            ffi::EN_getdemandpattern(self.ph, node_index, demand_index, &mut out_pattern)
        })?;
        Ok(out_pattern)
    }

    /// Sets the pattern index for a demand category on a node (0 = no pattern).
    pub fn set_demand_pattern(
        &self,
        node_index: i32,
        demand_index: i32,
        pattern_index: i32,
    ) -> Result<()> {
        check_error(unsafe {
            ffi::EN_setdemandpattern(self.ph, node_index, demand_index, pattern_index)
        })
    }

    /// Returns the name of a demand category on a node.
    pub fn get_demand_name(&self, node_index: i32, demand_index: i32) -> Result<String> {
        let mut out_name: Vec<c_char> = vec![0; MAX_ID_SIZE as usize + 1usize];
        check_error(unsafe {
            ffi::EN_getdemandname(self.ph, node_index, demand_index, out_name.as_mut_ptr())
        })?;
        let name = unsafe { std::ffi::CStr::from_ptr(out_name.as_ptr()) }
            .to_string_lossy()
            .trim_end()
            .to_string();
        Ok(name)
    }

    /// Sets the name of a demand category on a node.
    pub fn set_demand_name(
        &self,
        node_index: i32,
        demand_index: i32,
        name: &str,
    ) -> Result<()> {
        let c_name = CString::new(name).expect("Demand name contains null bytes");
        check_error(unsafe {
            ffi::EN_setdemandname(self.ph, node_index, demand_index, c_name.as_ptr())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use crate::types::demand::Demand;
    use rstest::rstest;

    #[rstest]
    fn test_create_demand(ph: EPANET) {
        let node_index = ph.get_node_index("12").unwrap();
        let demand = Demand::new(&ph, node_index, 50.0, "", "TestDemand").unwrap();

        assert_eq!(demand.node_index, node_index);
        assert_eq!(demand.base_demand, 50.0);
        assert_eq!(demand.name, "TestDemand");
        assert_eq!(demand.pattern_index, 0);
        assert!(demand.demand_index() > 0);
    }

    #[rstest]
    fn test_create_demand_with_pattern(ph: EPANET) {
        let node_index = ph.get_node_index("12").unwrap();

        // Create a pattern first
        ph.add_pattern("DemPat").unwrap();
        ph.set_pattern(
            ph.get_pattern_index("DemPat").unwrap(),
            &[1.0, 1.5, 0.5],
        )
        .unwrap();

        let demand = Demand::new(&ph, node_index, 75.0, "DemPat", "PatternedDemand").unwrap();

        assert_eq!(demand.base_demand, 75.0);
        assert_eq!(demand.name, "PatternedDemand");
        assert!(demand.pattern_index > 0);
    }

    #[rstest]
    fn test_get_demand_by_name(ph: EPANET) {
        let node_index = ph.get_node_index("12").unwrap();
        let _created = Demand::new(&ph, node_index, 100.0, "", "ByNameTest").unwrap();

        let fetched = ph.get_demand(node_index, "ByNameTest").unwrap();
        assert_eq!(fetched.name, "ByNameTest");
        assert!(approx_eq(fetched.base_demand, 100.0, 1e-6));
    }

    #[rstest]
    fn test_get_demand_by_index(ph: EPANET) {
        let node_index = ph.get_node_index("12").unwrap();
        let created = Demand::new(&ph, node_index, 200.0, "", "ByIdxTest").unwrap();
        let demand_index = created.demand_index();

        let fetched = ph.get_demand_by_index(node_index, demand_index).unwrap();
        assert_eq!(fetched.name, "ByIdxTest");
        assert!(approx_eq(fetched.base_demand, 200.0, 1e-6));
    }

    #[rstest]
    fn test_update_demand(ph: EPANET) {
        let node_index = ph.get_node_index("12").unwrap();
        let mut demand = Demand::new(&ph, node_index, 50.0, "", "UpdateTest").unwrap();

        // Update base demand
        demand.base_demand = 300.0;
        demand.update().unwrap();

        let fetched = ph.get_demand(node_index, "UpdateTest").unwrap();
        assert!(approx_eq(fetched.base_demand, 300.0, 1e-6));
    }

    #[rstest]
    fn test_update_demand_name(ph: EPANET) {
        let node_index = ph.get_node_index("12").unwrap();
        let mut demand = Demand::new(&ph, node_index, 50.0, "", "OldName").unwrap();

        demand.name = "NewName".to_string();
        demand.update().unwrap();

        let fetched = ph
            .get_demand_by_index(node_index, demand.demand_index())
            .unwrap();
        assert_eq!(fetched.name, "NewName");
    }

    #[rstest]
    fn test_delete_demand(ph: EPANET) {
        let node_index = ph.get_node_index("12").unwrap();
        let count_before = ph.get_demand_count(node_index).unwrap();

        let demand = Demand::new(&ph, node_index, 50.0, "", "DeleteMe").unwrap();
        assert_eq!(ph.get_demand_count(node_index).unwrap(), count_before + 1);

        demand.delete().unwrap();
        assert_eq!(ph.get_demand_count(node_index).unwrap(), count_before);
    }

    #[rstest]
    fn test_multiple_demands_on_node(ph: EPANET) {
        let node_index = ph.get_node_index("12").unwrap();
        let count_before = ph.get_demand_count(node_index).unwrap();

        let _d1 = Demand::new(&ph, node_index, 10.0, "", "First").unwrap();
        let _d2 = Demand::new(&ph, node_index, 20.0, "", "Second").unwrap();
        let _d3 = Demand::new(&ph, node_index, 30.0, "", "Third").unwrap();

        assert_eq!(
            ph.get_demand_count(node_index).unwrap(),
            count_before + 3
        );

        let fetched = ph.get_demand(node_index, "Second").unwrap();
        assert!(approx_eq(fetched.base_demand, 20.0, 1e-6));
    }

    #[rstest]
    fn test_get_existing_demand(ph: EPANET) {
        // net1.inp node "12" has one existing demand category
        let node_index = ph.get_node_index("12").unwrap();
        let demand = ph.get_demand_by_index(node_index, 1).unwrap();

        assert!(approx_eq(demand.base_demand, 150.0, 1e-6));
    }
}
