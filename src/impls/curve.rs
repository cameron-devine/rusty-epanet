//! Curve-related API methods for EPANET.
//!
//! This module contains methods for getting and adding curves.
use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::types::MAX_ID_SIZE;
use crate::types::curve::{Curve, CurveType};
use crate::EPANET;
use enum_primitive::FromPrimitive;

/// ## Curve APIs
impl EPANET {

    pub fn create_curve(&self, id: &str, curve_type: CurveType, points: &[(f64, f64)]) -> Result<Curve<'_>> {
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

    pub fn get_curve_by_id(&self, id: &str) -> Result<Curve<'_>> {
        let index = self.get_curve_index(id)?;
        let curve_type = self.get_curve_type(index)?;
        let points = self.get_curve_points(index)?;

        Ok(Curve {
            project: self,
            index,
            id: id.to_string(),
            curve_type,
            points,
        })
    }

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

    pub fn update_curve(&self, curve: &Curve) -> Result<()> {
        self.set_curve_id(curve.index, &curve.id)?;
        self.set_curve_type(curve.index, curve.curve_type)?;
        self.set_curve(curve.index, &curve.points)?;
        Ok(())
    }

    pub fn delete_curve(&self, curve: Curve) -> Result<()> {
        self.delete_curve_by_id(curve.index)?;
        Ok(())
    }

    fn add_curve(&self, id: &str) -> Result<()> {
        let c_id = std::ffi::CString::new(id).unwrap();
        let result = unsafe { ffi::EN_addcurve(self.ph, c_id.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn delete_curve_by_id(&self, index: i32) -> Result<()> {
        let result = unsafe { ffi::EN_deletecurve(self.ph, index) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn get_curve_index(&self, id: &str) -> Result<i32> {
        let c_id = std::ffi::CString::new(id).unwrap();
        let mut out_index = 0;
        let result = unsafe { ffi::EN_getcurveindex(self.ph, c_id.as_ptr(), &mut out_index) };
        if result == 0 {
            Ok(out_index)
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn get_curve_id(&self, index: i32) -> Result<String> {
        let mut out_id: Vec<std::ffi::c_char> = vec![0; MAX_ID_SIZE as usize + 1];
        let result = unsafe { ffi::EN_getcurveid(self.ph, index, out_id.as_mut_ptr()) };
        if result == 0 {
            let id = unsafe { std::ffi::CStr::from_ptr(out_id.as_ptr()) }
                .to_str()
                .unwrap_or("")
                .trim_end()
                .to_string();
            Ok(id)
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn set_curve_id(&self, index: i32, id: &str) -> Result<()> {
        let c_id = std::ffi::CString::new(id).unwrap();
        let result = unsafe { ffi::EN_setcurveid(self.ph, index, c_id.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn get_curve_len(&self, index: i32) -> Result<i32> {
        let mut out_len = 0;
        let result = unsafe { ffi::EN_getcurvelen(self.ph, index, &mut out_len) };
        if result == 0 {
            Ok(out_len)
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn get_curve_type(&self, index: i32) -> Result<CurveType> {
        let mut out_type = 0;
        let result = unsafe { ffi::EN_getcurvetype(self.ph, index, &mut out_type) };
        if result == 0 {
            Ok(CurveType::from_i32(out_type).unwrap())
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn set_curve_type(&self, index: i32, curve_type: CurveType) -> Result<()> {
        let result = unsafe { ffi::EN_setcurvetype(self.ph, index, curve_type as i32) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn get_curve_points(&self, index: i32) -> Result<Vec<(f64, f64)>> {
        let len = self.get_curve_len(index)?;
        let mut out_id: Vec<std::ffi::c_char> = vec![0; MAX_ID_SIZE as usize + 1];
        let mut out_x = vec![0.0; len as usize];
        let mut out_y = vec![0.0; len as usize];
        let mut out_len = 0;

        let result = unsafe {
            ffi::EN_getcurve(
                self.ph,
                index,
                out_id.as_mut_ptr(),
                &mut out_len,
                out_x.as_mut_ptr(),
                out_y.as_mut_ptr(),
            )
        };
        if result == 0 {
            Ok(out_x
                .iter()
                .zip(out_y.iter())
                .map(|(&a, &b)| (a, b))
                .collect())
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn set_curve(&self, index: i32, values: &[(f64, f64)]) -> Result<()> {
        let (mut x_vec, mut y_vec): (Vec<f64>, Vec<f64>) = values.iter().cloned().unzip();

        let result = unsafe {
            ffi::EN_setcurve(
                self.ph,
                index,
                x_vec.as_mut_ptr(),
                y_vec.as_mut_ptr(),
                values.len() as i32,
            )
        };
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
    use crate::EPANET;
    use crate::impls::test_utils::fixtures::*;
    use rstest::*;

    #[rstest]
    fn test_create_and_get_curve(ph: EPANET) {
        let points = vec![(1.0, 2.0), (3.0, 4.0)];
        let id = "CurveA";
        let curve_type = CurveType::EfficCurve;

        let curve = ph.create_curve(id, curve_type, &points).unwrap();
        assert_eq!(curve.id, id);
        assert_eq!(curve.curve_type, curve_type);
        assert_eq!(curve.points, points);

        let fetched = ph.get_curve_by_id(id).unwrap();
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.curve_type, curve_type);
        assert_eq!(fetched.points, points);
    }

    #[rstest]
    fn test_update_curve(ph: EPANET) {
        let id = "CurveB";
        let curve_type = CurveType::PumpCurve;
        let points = vec![(5.0, 6.0), (7.0, 8.0)];
        let mut curve = ph.create_curve(id, curve_type, &points).unwrap();

        // Update curve and sync changes
        curve.points = vec![(9.0, 10.0), (11.0, 12.0)];
        curve.update().unwrap();

        let updated = ph.get_curve_by_id(id).unwrap();
        assert_eq!(updated.points, curve.points);
    }

    #[rstest]
    fn test_delete_curve(ph: EPANET) {
        let id = "CurveC";
        let curve_type = CurveType::VolumeCurve;
        let points = vec![(13.0, 14.0)];
        let curve = ph.create_curve(id, curve_type, &points).unwrap();

        curve.delete().unwrap();
        let result = ph.get_curve_by_id(id);
        assert!(result.is_err());
    }
}
