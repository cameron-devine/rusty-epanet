//! Curve-related API methods for EPANET.
//!
//! This module contains methods for getting and adding curves.
use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::curve::{Curve, CurveType};
use crate::types::MAX_ID_SIZE;
use crate::EPANET;
use num_traits::FromPrimitive;
use crate::ffi::{EN_getcurvevalue, EN_setcurvevalue};

/// ## Curve APIs
impl EPANET {
    /// Creates a new curve in the EPANET model.
    ///
    /// # Parameters
    /// - `id`: Unique identifier for the curve
    /// - `curve_type`: Type of curve (volume, pump, efficiency, etc.)
    /// - `points`: Array of (x, y) coordinate pairs defining the curve
    ///
    /// # Returns
    /// A [`Curve`] struct with a reference to this project.
    pub fn create_curve(
        &self,
        id: &str,
        curve_type: CurveType,
        points: &[(f64, f64)],
    ) -> Result<Curve<'_>> {
        self.add_curve(id)?;

        let index = self.get_curve_index(id)?;
        self.set_curve_type(index, curve_type)?;
        self.set_curve(index, points)?;

        Ok(Curve {
            project: self,
            index,
            id: id.to_string(),
            curve_type,
            points: points.to_vec(),
        })
    }

    /// Retrieves a curve by its ID.
    ///
    /// Convenience method that calls [`get_curve_by_id`](Self::get_curve_by_id).
    pub fn get_curve(&self, id: &str) -> Result<Curve<'_>> {
        self.get_curve_by_id(id)
    }

    /// Retrieves a curve by its ID.
    pub fn get_curve_by_id(&self, id: &str) -> Result<Curve<'_>> {
        let index = self.get_curve_index(id)?;
        self.get_curve_by_index(index)
    }

    /// Retrieves a curve by its index.
    pub fn get_curve_by_index(&self, index: i32) -> Result<Curve<'_>> {
        let id = self.get_curve_id(index)?;
        let curve_type = self.get_curve_type(index)?;
        let points = self.get_curve_points(index)?;

        Ok(Curve {
            project: self,
            index,
            id,
            curve_type,
            points,
        })
    }

    // Helper methods - Internal API

    pub fn add_curve(&self, id: &str) -> Result<()> {
        let c_id = std::ffi::CString::new(id).unwrap();
        check_error(unsafe { ffi::EN_addcurve(self.ph, c_id.as_ptr()) })
    }

    pub fn delete_curve_by_id(&self, index: i32) -> Result<()> {
        check_error(unsafe { ffi::EN_deletecurve(self.ph, index) })
    }

    pub fn get_curve_index(&self, id: &str) -> Result<i32> {
        let c_id = std::ffi::CString::new(id).unwrap();
        let mut out_index = 0;
        check_error(unsafe { ffi::EN_getcurveindex(self.ph, c_id.as_ptr(), &mut out_index) })?;
        Ok(out_index)
    }

    pub fn get_curve_id(&self, index: i32) -> Result<String> {
        let mut out_id: Vec<std::ffi::c_char> = vec![0; MAX_ID_SIZE as usize + 1];
        check_error(unsafe { ffi::EN_getcurveid(self.ph, index, out_id.as_mut_ptr()) })?;
        let id = unsafe { std::ffi::CStr::from_ptr(out_id.as_ptr()) }
            .to_str()
            .unwrap_or("")
            .trim_end()
            .to_string();
        Ok(id)
    }

    pub fn set_curve_id(&self, index: i32, id: &str) -> Result<()> {
        let c_id = std::ffi::CString::new(id).unwrap();
        check_error(unsafe { ffi::EN_setcurveid(self.ph, index, c_id.as_ptr()) })
    }

    pub fn get_curve_len(&self, index: i32) -> Result<i32> {
        let mut out_len = 0;
        check_error(unsafe { ffi::EN_getcurvelen(self.ph, index, &mut out_len) })?;
        Ok(out_len)
    }

    pub fn get_curve_type(&self, index: i32) -> Result<CurveType> {
        let mut out_type = 0;
        check_error(unsafe { ffi::EN_getcurvetype(self.ph, index, &mut out_type) })?;
        Ok(CurveType::from_i32(out_type).unwrap())
    }

    pub fn set_curve_type(&self, index: i32, curve_type: CurveType) -> Result<()> {
        check_error(unsafe { ffi::EN_setcurvetype(self.ph, index, curve_type as i32) })
    }

    pub fn get_curve_points(&self, index: i32) -> Result<Vec<(f64, f64)>> {
        let len = self.get_curve_len(index)?;
        let mut out_id: Vec<std::ffi::c_char> = vec![0; MAX_ID_SIZE as usize + 1];
        let mut out_x = vec![0.0; len as usize];
        let mut out_y = vec![0.0; len as usize];
        let mut out_len = 0;

        check_error(unsafe {
            ffi::EN_getcurve(
                self.ph,
                index,
                out_id.as_mut_ptr(),
                &mut out_len,
                out_x.as_mut_ptr(),
                out_y.as_mut_ptr(),
            )
        })?;
        Ok(out_x
            .iter()
            .zip(out_y.iter())
            .map(|(&a, &b)| (a, b))
            .collect())
    }

    pub fn set_curve(&self, index: i32, values: &[(f64, f64)]) -> Result<()> {
        let (mut x_vec, mut y_vec): (Vec<f64>, Vec<f64>) = values.iter().cloned().unzip();

        check_error(unsafe {
            ffi::EN_setcurve(
                self.ph,
                index,
                x_vec.as_mut_ptr(),
                y_vec.as_mut_ptr(),
                values.len() as i32,
            )
        })
    }

    pub fn get_curve_value(&self, index: i32, point_index: i32) -> Result<(f64, f64)> {
        let (mut out_x, mut out_y) = (0f64, 0f64);
        check_error( unsafe { EN_getcurvevalue(self.ph, index, point_index, &mut out_x, &mut out_y)})?;
        Ok((out_x, out_y))
    }

    pub fn set_curve_value(&self, index: i32, point_index: i32, point: (f64, f64)) -> Result<()> {
        check_error(unsafe{ EN_setcurvevalue(self.ph, index, point_index, point.0, point.1) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use crate::types::curve::Curve;
    use crate::EPANET;
    use rstest::*;

    #[rstest]
    fn test_create_curve_constructors(ph: EPANET) {
        // Test volume curve
        let vol_points = vec![(0.0, 0.0), (10.0, 1000.0), (20.0, 5000.0)];
        let vol_curve = Curve::new_volume_curve(&ph, "VolCurve1", &vol_points).unwrap();
        assert_eq!(vol_curve.id, "VolCurve1");
        assert_eq!(vol_curve.curve_type, CurveType::VolumeCurve);
        assert_eq!(vol_curve.points, vol_points);

        // Test pump curve
        let pump_points = vec![(0.0, 100.0), (500.0, 80.0), (1000.0, 50.0)];
        let pump_curve = Curve::new_pump_curve(&ph, "PumpCurve1", &pump_points).unwrap();
        assert_eq!(pump_curve.curve_type, CurveType::PumpCurve);
        assert_eq!(pump_curve.points, pump_points);

        // Test efficiency curve
        let eff_points = vec![(0.0, 0.0), (500.0, 85.0), (1000.0, 75.0)];
        let eff_curve = Curve::new_efficiency_curve(&ph, "EffCurve1", &eff_points).unwrap();
        assert_eq!(eff_curve.curve_type, CurveType::EfficCurve);

        // Test headloss curve
        let hloss_points = vec![(0.0, 0.0), (100.0, 5.0)];
        let hloss_curve = Curve::new_headloss_curve(&ph, "HLossCurve1", &hloss_points).unwrap();
        assert_eq!(hloss_curve.curve_type, CurveType::HLossCurve);

        // Test generic curve
        let gen_points = vec![(1.0, 1.0), (2.0, 4.0)];
        let gen_curve = Curve::new_generic_curve(&ph, "GenCurve1", &gen_points).unwrap();
        assert_eq!(gen_curve.curve_type, CurveType::GenericCurve);

        // Test valve curve
        let valve_points = vec![(0.0, 0.0), (0.5, 0.25), (1.0, 1.0)];
        let valve_curve = Curve::new_valve_curve(&ph, "ValveCurve1", &valve_points).unwrap();
        assert_eq!(valve_curve.curve_type, CurveType::ValveCurve);
    }

    #[rstest]
    fn test_get_curve_methods(ph: EPANET) {
        let points = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)];
        let id = "TestCurve";
        let curve_type = CurveType::PumpCurve;

        // Create curve
        let created = ph.create_curve(id, curve_type, &points).unwrap();
        let index = created.index;

        // Test get_curve (convenience method)
        let curve1 = ph.get_curve(id).unwrap();
        assert_eq!(curve1.id, id);
        assert_eq!(curve1.curve_type, curve_type);
        assert_eq!(curve1.points, points);

        // Test get_curve_by_id
        let curve2 = ph.get_curve_by_id(id).unwrap();
        assert_eq!(curve2.id, id);
        assert_eq!(curve2.index, index);

        // Test get_curve_by_index
        let curve3 = ph.get_curve_by_index(index).unwrap();
        assert_eq!(curve3.id, id);
        assert_eq!(curve3.curve_type, curve_type);
        assert_eq!(curve3.points, points);
    }

    #[rstest]
    fn test_update_curve(ph: EPANET) {
        let id = "UpdateTest";
        let points = vec![(0.0, 0.0), (10.0, 100.0)];
        let mut curve = Curve::new_pump_curve(&ph, id, &points).unwrap();

        // Update ID
        curve.id = "UpdatedID".to_string();
        curve.update().unwrap();
        let fetched = ph.get_curve("UpdatedID").unwrap();
        assert_eq!(fetched.id, "UpdatedID");

        // Update curve type
        curve.curve_type = CurveType::EfficCurve;
        curve.update().unwrap();
        let fetched = ph.get_curve("UpdatedID").unwrap();
        assert_eq!(fetched.curve_type, CurveType::EfficCurve);

        // Update points
        let new_points = vec![(0.0, 0.0), (50.0, 75.0), (100.0, 90.0)];
        curve.points = new_points.clone();
        curve.update().unwrap();
        let fetched = ph.get_curve("UpdatedID").unwrap();
        assert_eq!(fetched.points, new_points);
    }

    #[rstest]
    fn test_delete_curve(ph: EPANET) {
        let points = vec![(0.0, 0.0), (10.0, 100.0)];
        let curve = Curve::new_volume_curve(&ph, "DeleteTest", &points).unwrap();
        let id = curve.id.clone();

        // Verify curve exists
        assert!(ph.get_curve(&id).is_ok());

        // Delete curve
        curve.delete().unwrap();

        // Verify curve no longer exists
        let result = ph.get_curve(&id);
        assert!(result.is_err());
    }
}
