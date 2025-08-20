use crate::{bindings::*, EPANET};
use enum_primitive::*;

/// A struct representing a curve in an EPANET project.
///
/// Curves now hold a reference to their parent [`EPANET`] project so that
/// changes can be synchronised back to the engine without having to invoke
/// update functions on the project explicitly. Calling [`Curve::update`] will
/// commit local field changes to EPANET.
#[derive(Debug, Clone)]
pub struct Curve<'a> {
    /// Reference to the owning EPANET project
    pub(crate) project: &'a EPANET,
    /// EPANET project index of the curve
    pub(crate) index: i32,
    /// Curve ID
    pub id: String,
    /// Curve type of type [`CurveType`]
    pub curve_type: CurveType,
    /// Curve points given as a vector of (x, y) tuples
    pub points: Vec<(f64, f64)>,
}

impl<'a> Curve<'a> {
    /// Returns the EPANET project index of the curve
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Synchronises any local changes of this curve back to the EPANET engine.
    pub fn update(&self) -> crate::epanet_error::Result<()> {
        self.project.update_curve(self)
    }

    /// Deletes this curve from the EPANET project.
    pub fn delete(self) -> crate::epanet_error::Result<()> {
        self.project.delete_curve(self)
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