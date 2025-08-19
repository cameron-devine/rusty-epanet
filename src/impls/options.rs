//! Analysis options API methods for EPANET.
//!
//! This module contains methods for setting and reading analysis options.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::types::{
    FlowUnits, Option, QualityAnalysisInfo, QualityType, TimeParameter, MAX_ID_SIZE,
};
use crate::EPANET;
use enum_primitive::FromPrimitive;
use std::ffi::{c_char, CString};

/// ## Analysis Options APIs
impl EPANET {
    pub fn get_option(&self, option: Option) -> Result<f64> {
        let mut value: f64 = 0.0;
        let result = unsafe { ffi::EN_getoption(self.ph, option as i32, &mut value) };
        if result == 0 {
            Ok(value)
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_option(&self, option: Option, value: f64) -> Result<()> {
        let result = unsafe { ffi::EN_setoption(self.ph, option as i32, value) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_flow_units(&self) -> Result<FlowUnits> {
        let mut flow_units = 0; // Default value
        let result = unsafe { ffi::EN_getflowunits(self.ph, &mut flow_units) };
        if result == 0 {
            Ok(FlowUnits::from_i32(flow_units).unwrap())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_flow_units(&self, flow_units: FlowUnits) -> Result<()> {
        let result = unsafe { ffi::EN_setflowunits(self.ph, flow_units as i32) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_time_parameter(&self, parameter: TimeParameter) -> Result<i64> {
        let mut value: i64 = 0;
        let result = unsafe { ffi::EN_gettimeparam(self.ph, parameter as i32, &mut value) };
        if result == 0 {
            Ok(value)
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_time_parameter(&self, parameter: TimeParameter, value: i64) -> Result<()> {
        let result = unsafe { ffi::EN_settimeparam(self.ph, parameter as i32, value) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_quality_info(&self) -> Result<QualityAnalysisInfo> {
        let mut quality_type: i32 = 0;
        let mut chem_name = vec![0 as c_char; MAX_ID_SIZE as usize + 1];
        let mut chem_units = vec![0 as c_char; MAX_ID_SIZE as usize + 1];
        let mut trace_node_index: i32 = 0;

        let result = unsafe {
            ffi::EN_getqualinfo(
                self.ph,
                &mut quality_type,
                chem_name.as_mut_ptr(),
                chem_units.as_mut_ptr(),
                &mut trace_node_index,
            )
        };

        if result != 0 {
            return Err(EPANETError::from(result));
        }

        let quality_type =
            QualityType::from_i32(quality_type).ok_or_else(|| EPANETError::from(result))?;

        let chem_name = unsafe {
            std::ffi::CStr::from_ptr(chem_name.as_ptr())
                .to_string_lossy()
                .trim_end()
                .to_string()
        };

        let chem_units = unsafe {
            std::ffi::CStr::from_ptr(chem_units.as_ptr())
                .to_string_lossy()
                .trim_end()
                .to_string()
        };

        Ok(QualityAnalysisInfo {
            quality_type,
            chem_name,
            chem_units,
            trace_node_index,
        })
    }

    pub fn get_quality_type(&self) -> Result<QualityType> {
        let mut quality_type: i32 = 0;
        let mut trace_node_index: i32 = 0;

        let result =
            unsafe { ffi::EN_getqualtype(self.ph, &mut quality_type, &mut trace_node_index) };
        if result == 0 {
            Ok(QualityType::from_i32(quality_type).unwrap())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn set_quality_type(
        &self,
        quality_type: QualityType,
        chem_name: &str,
        chem_units: &str,
        trace_node: &str,
    ) -> Result<()> {
        let c_chem_name = CString::new(chem_name).expect("Title contains null bytes");
        let c_chem_units = CString::new(chem_units).expect("Title contains null bytes");
        let c_trace_node = CString::new(trace_node).expect("Title contains null bytes");

        let result = unsafe {
            ffi::EN_setqualtype(
                self.ph,
                quality_type as i32,
                c_chem_name.as_ptr(),
                c_chem_units.as_ptr(),
                c_trace_node.as_ptr(),
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
    use crate::impls::test_utils::fixtures::ph;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    #[rstest]
    fn test_get_options(ph: EPANET) {
        let ref_values = vec![
            40.0, 0.001, 0.01, 0.5, 1.0, 0.0, 0.0, 0.0, 75.0, 0.0, 0.0, 0.0, 1.0, 1.0, 10.0, 2.0,
            10.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0,
        ];
        let mut test_values = Vec::new();

        let mut result = ph.solve_h();
        assert!(result.is_ok());

        result = ph.solve_q();
        assert!(result.is_ok());

        for opt in Option::iter().take(Option::iter().count() - 1) {
            let option_result = ph.get_option(opt);
            assert!(option_result.is_ok());

            let value = option_result.unwrap();
            test_values.push(value);
        }

        assert_eq!(test_values, ref_values);
    }

    #[rstest]
    fn test_get_time_param(ph: EPANET) {
        let mut test_values = Vec::new();
        let ref_values = vec![
            86400, 3600, 300, 7200, 0, 3600, 0, 360, 0, 25, 0, 86400, 86400, 0, 3600, 0,
        ];

        let mut result = ph.solve_h();
        assert!(result.is_ok());

        result = ph.solve_q();
        assert!(result.is_ok());

        for time_param in TimeParameter::iter() {
            let tp_result = ph.get_time_parameter(time_param);
            assert!(tp_result.is_ok());

            let value = tp_result.unwrap();
            test_values.push(value);
        }

        assert_eq!(test_values, ref_values);
    }
}
