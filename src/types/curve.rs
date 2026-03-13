use crate::{bindings::*, EPANET, epanet_error::*};
use num_derive::FromPrimitive;

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
    /// Creates a new volume curve (tank volume vs. depth).
    pub fn new_volume_curve(
        project: &'a EPANET,
        id: &str,
        points: &[(f64, f64)],
    ) -> Result<Self> {
        project.create_curve(id, CurveType::VolumeCurve, points)
    }

    /// Creates a new pump curve (head vs. flow).
    pub fn new_pump_curve(
        project: &'a EPANET,
        id: &str,
        points: &[(f64, f64)],
    ) -> Result<Self> {
        project.create_curve(id, CurveType::PumpCurve, points)
    }

    /// Creates a new efficiency curve (pump efficiency vs. flow).
    pub fn new_efficiency_curve(
        project: &'a EPANET,
        id: &str,
        points: &[(f64, f64)],
    ) -> Result<Self> {
        project.create_curve(id, CurveType::EfficCurve, points)
    }

    /// Creates a new head loss curve (valve head loss vs. flow).
    pub fn new_headloss_curve(
        project: &'a EPANET,
        id: &str,
        points: &[(f64, f64)],
    ) -> Result<Self> {
        project.create_curve(id, CurveType::HLossCurve, points)
    }

    /// Creates a new generic curve.
    pub fn new_generic_curve(
        project: &'a EPANET,
        id: &str,
        points: &[(f64, f64)],
    ) -> Result<Self> {
        project.create_curve(id, CurveType::GenericCurve, points)
    }

    /// Creates a new valve curve (valve loss coefficient vs. fraction open).
    pub fn new_valve_curve(
        project: &'a EPANET,
        id: &str,
        points: &[(f64, f64)],
    ) -> Result<Self> {
        project.create_curve(id, CurveType::ValveCurve, points)
    }

    /// Updates the curve in the EPANET model with current field values.
    pub fn update(&self) -> Result<()> {
        let current_id = self.project.get_curve_id(self.index)?;
        if current_id != self.id {
            self.project.set_curve_id(self.index, &self.id)?;
        }

        self.project.set_curve_type(self.index, self.curve_type)?;
        self.project.set_curve(self.index, &self.points)
    }

    /// Deletes this curve from the EPANET model.
    ///
    /// This method consumes the curve, preventing further use after deletion.
    pub fn delete(self) -> Result<()> {
        self.project.delete_curve_by_id(self.index)
    }
}

/// Represents the type of a curve in an EPANET project.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum CurveType {
    /// Tank volume vs. depth curve
    VolumeCurve = EN_CurveType_EN_VOLUME_CURVE as i32,
    /// Pump head vs. flow curve
    PumpCurve = EN_CurveType_EN_PUMP_CURVE as i32,
    /// Pump efficiency vs. flow curve
    EfficCurve = EN_CurveType_EN_EFFIC_CURVE as i32,
    /// Valve head loss vs. flow curve
    HLossCurve = EN_CurveType_EN_HLOSS_CURVE as i32,
    /// Generic curve
    GenericCurve = EN_CurveType_EN_GENERIC_CURVE as i32,
    /// Valve loss coefficient vs. fraction open
    ValveCurve = EN_CurveType_EN_VALVE_CURVE as i32,
}
