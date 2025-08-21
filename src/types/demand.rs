use crate::bindings::*;
use enum_primitive::*;

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum DemandModel {
    Dda = EN_DemandModel_EN_DDA, // Demand driven analysis
    Pda = EN_DemandModel_EN_PDA, // Pressure driven analysis
}}

pub struct DemandModelInfo {
    pub demand_type: DemandModel,
    pub pressure_min: f64,
    pub pressure_required: f64,
    pub pressure_exponent: f64,
}
