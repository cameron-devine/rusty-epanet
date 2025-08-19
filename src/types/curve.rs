use crate::bindings::*;
use enum_primitive::*;

/// A struct representing a curve in an EPANET project.
/// Use [`crate::EPANET::create_curve`] on the EPANET project
/// to create a curve. If needing to modify a curve,
/// ensure you call [`crate::EPANET::update_curve`] on the EPANET
/// project after altering the curve internals.
#[derive(Debug, Clone)]
pub struct Curve {
    /// EPANET project index of the curve
    pub(crate) index: i32,
    /// Curve ID
    pub id: String,
    /// Curve type of type [`CurveType`]
    pub curve_type: CurveType,
    /// Curve points given as a vector of (x, y) tuples
    pub points: Vec<(f64, f64)>,
}

impl Curve {
    /// Returns the EPANET project index of the curve
    pub fn index(&self) -> i32 {
        self.index
    }
}

enum_from_primitive! {
    /// Represents the type of a curve in an EPANET project.
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum CurveType {
        /// Tank volume vs. depth curve
        VolumeCurve = EN_CurveType_EN_VOLUME_CURVE,
        /// Pump head vs. flow curve
        PumpCurve = EN_CurveType_EN_PUMP_CURVE,
        /// Pump efficiency vs. flow curve
        EfficCurve = EN_CurveType_EN_EFFIC_CURVE,
        /// Valve head loss vs. flow curve
        HLossCurve = EN_CurveType_EN_HLOSS_CURVE,
        /// Generic curve
        GenericCurve = EN_CurveType_EN_GENERIC_CURVE,
        /// Valve loss coefficient vs. fraction open
        ValveCurve = EN_CurveType_EN_VALVE_CURVE,
    }
}