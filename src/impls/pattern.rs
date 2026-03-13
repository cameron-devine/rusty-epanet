//! Pattern APIs
//!
//! This module contains low-level APIs for managing time patterns in EPANET,
//! as well as higher-level constructors that return [`Pattern`] structs.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::pattern::Pattern;
use crate::types::MAX_ID_SIZE;
use crate::EPANET;
use std::path::Path;

/// ## Pattern APIs
impl EPANET {
    /// Creates a new time pattern in the EPANET model and returns a [`Pattern`] struct.
    ///
    /// # Parameters
    ///
    /// - `id`: Unique identifier for the pattern
    /// - `multipliers`: Array of multipliers, one per time period
    ///
    /// # Returns
    ///
    /// A [`Pattern`] struct with a reference to this project.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the pattern cannot be created (e.g., duplicate ID).
    pub fn create_pattern(&self, id: &str, multipliers: &[f64]) -> Result<Pattern<'_>> {
        self.add_pattern(id)?;
        let index = self.get_pattern_index(id)?;
        self.set_pattern(index, multipliers)?;

        Ok(Pattern {
            project: self,
            index,
            id: id.to_string(),
            multipliers: multipliers.to_vec(),
        })
    }

    /// Retrieves a pattern by its ID.
    ///
    /// Convenience method that resolves the ID to an index and calls
    /// [`get_pattern_by_index`](Self::get_pattern_by_index).
    ///
    /// # Parameters
    ///
    /// - `id`: The pattern's unique identifier
    ///
    /// # Returns
    ///
    /// A [`Pattern`] struct with all fields populated from the C engine.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the pattern does not exist.
    pub fn get_pattern(&self, id: &str) -> Result<Pattern<'_>> {
        let index = self.get_pattern_index(id)?;
        self.get_pattern_by_index(index)
    }

    /// Retrieves a pattern by its 1-based index.
    ///
    /// # Parameters
    ///
    /// - `index`: The pattern's 1-based index in the EPANET model
    ///
    /// # Returns
    ///
    /// A [`Pattern`] struct with all fields populated from the C engine.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the index is invalid.
    pub fn get_pattern_by_index(&self, index: i32) -> Result<Pattern<'_>> {
        let id = self.get_pattern_id(index)?;
        let length = self.get_pattern_length(index)?;

        let mut multipliers = Vec::with_capacity(length as usize);
        for period in 1..=length {
            multipliers.push(self.get_pattern_value(index, period)?);
        }

        Ok(Pattern {
            project: self,
            index,
            id,
            multipliers,
        })
    }

    // =========================================================================
    // Low-level FFI wrappers
    // =========================================================================

    /// Adds a new empty time pattern to the project.
    pub fn add_pattern(&self, id: &str) -> Result<()> {
        let c_id = std::ffi::CString::new(id).unwrap();
        check_error(unsafe { ffi::EN_addpattern(self.ph, c_id.as_ptr()) })
    }

    /// Deletes a time pattern from the project by its 1-based index.
    pub fn delete_pattern(&self, index: i32) -> Result<()> {
        check_error(unsafe { ffi::EN_deletepattern(self.ph, index) })
    }

    /// Returns the 1-based index of a pattern given its ID.
    pub fn get_pattern_index(&self, id: &str) -> Result<i32> {
        let mut index: i32 = 0;
        let c_id = std::ffi::CString::new(id).unwrap();
        check_error(unsafe { ffi::EN_getpatternindex(self.ph, c_id.as_ptr(), &mut index) })?;
        Ok(index)
    }

    /// Returns the ID of a pattern given its 1-based index.
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

    /// Sets the ID of a pattern given its 1-based index.
    pub fn set_pattern_id(&self, index: i32, id: &str) -> Result<()> {
        let c_id = std::ffi::CString::new(id).unwrap();
        check_error(unsafe { ffi::EN_setpatternid(self.ph, index, c_id.as_ptr()) })
    }

    /// Returns the number of time periods in a pattern.
    pub fn get_pattern_length(&self, index: i32) -> Result<i32> {
        let mut out_length = 0;
        check_error(unsafe { ffi::EN_getpatternlen(self.ph, index, &mut out_length) })?;
        Ok(out_length)
    }

    /// Returns the multiplier for a specific time period (1-based) of a pattern.
    pub fn get_pattern_value(&self, index: i32, period: i32) -> Result<f64> {
        let mut out_value = 0.0;
        check_error(unsafe { ffi::EN_getpatternvalue(self.ph, index, period, &mut out_value) })?;
        Ok(out_value)
    }

    /// Sets the multiplier for a specific time period (1-based) of a pattern.
    pub fn set_pattern_value(&self, index: i32, period: i32, value: f64) -> Result<()> {
        check_error(unsafe { ffi::EN_setpatternvalue(self.ph, index, period, value) })
    }

    /// Returns the average of all multipliers in a pattern.
    pub fn get_average_pattern_value(&self, index: i32) -> Result<f64> {
        let mut out_value = 0.0;
        check_error(unsafe { ffi::EN_getaveragepatternvalue(self.ph, index, &mut out_value) })?;
        Ok(out_value)
    }

    /// Sets all multipliers for a pattern at once, replacing any existing values.
    pub fn set_pattern(&self, index: i32, values: &[f64]) -> Result<()> {
        let c_values = values.as_ptr() as *mut f64;
        check_error(unsafe { ffi::EN_setpattern(self.ph, index, c_values, values.len() as i32) })
    }

    /// Loads a pattern from a file.
    ///
    /// # Parameters
    ///
    /// - `file_name`: Path to the pattern file
    /// - `id`: ID to assign to the loaded pattern
    pub fn load_pattern_file(&self, file_name: &Path, id: &str) -> Result<()> {
        let c_file_name = std::ffi::CString::new(file_name.to_str().unwrap()).unwrap();
        let c_id = std::ffi::CString::new(id).unwrap();

        check_error(unsafe {
            ffi::EN_loadpatternfile(self.ph, c_file_name.as_ptr(), c_id.as_ptr())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use crate::types::pattern::Pattern;
    use rstest::rstest;

    #[rstest]
    fn test_create_pattern(ph: EPANET) {
        let multipliers = vec![1.0, 1.2, 0.8, 0.6];
        let pattern = Pattern::new(&ph, "TestPat", &multipliers).unwrap();

        assert_eq!(pattern.id, "TestPat");
        assert_eq!(pattern.multipliers, multipliers);
        assert!(pattern.index() > 0);
    }

    #[rstest]
    fn test_create_pattern_via_project(ph: EPANET) {
        let multipliers = vec![0.5, 1.0, 1.5, 2.0, 1.5, 1.0];
        let pattern = ph.create_pattern("DemandPat", &multipliers).unwrap();

        assert_eq!(pattern.id, "DemandPat");
        assert_eq!(pattern.multipliers, multipliers);
    }

    #[rstest]
    fn test_get_pattern_by_id(ph: EPANET) {
        let multipliers = vec![1.0, 2.0, 3.0];
        let created = ph.create_pattern("GetTest", &multipliers).unwrap();
        let index = created.index();

        let fetched = ph.get_pattern("GetTest").unwrap();
        assert_eq!(fetched.id, "GetTest");
        assert_eq!(fetched.index(), index);
        assert_eq!(fetched.multipliers, multipliers);
    }

    #[rstest]
    fn test_get_pattern_by_index(ph: EPANET) {
        let multipliers = vec![0.8, 1.2];
        let created = ph.create_pattern("IdxTest", &multipliers).unwrap();
        let index = created.index();

        let fetched = ph.get_pattern_by_index(index).unwrap();
        assert_eq!(fetched.id, "IdxTest");
        assert_eq!(fetched.multipliers, multipliers);
    }

    #[rstest]
    fn test_update_pattern_id(ph: EPANET) {
        let multipliers = vec![1.0, 1.0, 1.0];
        let mut pattern = ph.create_pattern("OldName", &multipliers).unwrap();

        pattern.id = "NewName".to_string();
        pattern.update().unwrap();

        let fetched = ph.get_pattern("NewName").unwrap();
        assert_eq!(fetched.id, "NewName");
        assert_eq!(fetched.multipliers, multipliers);
    }

    #[rstest]
    fn test_update_pattern_multipliers(ph: EPANET) {
        let _pattern = ph.create_pattern("UpdateMult", &[1.0, 1.0]).unwrap();
        let mut pattern = ph.get_pattern("UpdateMult").unwrap();

        let new_multipliers = vec![0.5, 1.5, 2.0, 0.3];
        pattern.multipliers = new_multipliers.clone();
        pattern.update().unwrap();

        let fetched = ph.get_pattern("UpdateMult").unwrap();
        assert_eq!(fetched.multipliers, new_multipliers);

        // Drop the original pattern to avoid unused variable warning
        drop(pattern);
    }

    #[rstest]
    fn test_delete_pattern(ph: EPANET) {
        let pattern = ph.create_pattern("DeleteMe", &[1.0, 2.0, 3.0]).unwrap();
        let id = pattern.id.clone();

        // Verify it exists
        assert!(ph.get_pattern(&id).is_ok());

        // Delete it
        pattern.delete().unwrap();

        // Verify it no longer exists
        assert!(ph.get_pattern(&id).is_err());
    }

    #[rstest]
    fn test_pattern_average(ph: EPANET) {
        let multipliers = vec![1.0, 2.0, 3.0, 4.0];
        let pattern = ph.create_pattern("AvgTest", &multipliers).unwrap();

        let avg = pattern.average().unwrap();
        assert!(
            approx_eq(avg, 2.5, 1e-6),
            "Expected average 2.5, got {}",
            avg
        );
    }

    #[rstest]
    fn test_pattern_single_multiplier(ph: EPANET) {
        let pattern = ph.create_pattern("SingleMult", &[1.5]).unwrap();

        assert_eq!(pattern.multipliers, vec![1.5]);
        assert!(approx_eq(pattern.average().unwrap(), 1.5, 1e-6));
    }
}
