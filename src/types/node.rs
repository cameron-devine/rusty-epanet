use crate::bindings::*;
use enum_primitive::*;

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum MixingModel {
    Mix1 = EN_MixingModel_EN_MIX1, // Complete mix model
    Mix2 = EN_MixingModel_EN_MIX2, // 2-compartment model
    Fifo = EN_MixingModel_EN_FIFO, // First in, first out model
    Lifo = EN_MixingModel_EN_LIFO, // Last in, first out model
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum NodeProperty {
    Elevation = EN_NodeProperty_EN_ELEVATION, // Elevation
    BaseDemand = EN_NodeProperty_EN_BASEDEMAND, // Primary demand baseline value
    Pattern = EN_NodeProperty_EN_PATTERN, // Primary demand time pattern index
    Emitter = EN_NodeProperty_EN_EMITTER, // Emitter flow coefficient
    InitQual = EN_NodeProperty_EN_INITQUAL, // Initial quality
    SourceQual = EN_NodeProperty_EN_SOURCEQUAL, // Quality source strength
    SourcePat = EN_NodeProperty_EN_SOURCEPAT, // Quality source pattern index
    SourceType = EN_NodeProperty_EN_SOURCETYPE, // Quality source type
    TankLevel = EN_NodeProperty_EN_TANKLEVEL, // Current computed tank water level (read only)
    Demand = EN_NodeProperty_EN_DEMAND, // Current computed demand (read only)
    Head = EN_NodeProperty_EN_HEAD, // Current computed hydraulic head (read only)
    Pressure = EN_NodeProperty_EN_PRESSURE, // Current computed pressure (read only)
    Quality = EN_NodeProperty_EN_QUALITY, // Current computed quality (read only)
    SourceMass = EN_NodeProperty_EN_SOURCEMASS, // Current computed quality source mass inflow (read only)
    InitVolume = EN_NodeProperty_EN_INITVOLUME, // Tank initial volume (read only)
    MixModel = EN_NodeProperty_EN_MIXMODEL, // Tank mixing model
    MixZoneVol = EN_NodeProperty_EN_MIXZONEVOL, // Tank mixing zone volume (read only)
    TankDiam = EN_NodeProperty_EN_TANKDIAM, // Tank diameter
    MinVolume = EN_NodeProperty_EN_MINVOLUME, // Tank minimum volume
    VolCurve = EN_NodeProperty_EN_VOLCURVE, // Tank volume curve index
    MinLevel = EN_NodeProperty_EN_MINLEVEL, // Tank minimum level
    MaxLevel = EN_NodeProperty_EN_MAXLEVEL, // Tank maximum level
    MixFraction = EN_NodeProperty_EN_MIXFRACTION, // Tank mixing fraction
    TankKBulk = EN_NodeProperty_EN_TANK_KBULK, // Tank bulk decay coefficient
    TankVolume = EN_NodeProperty_EN_TANKVOLUME, // Current computed tank volume (read only)
    MaxVolume = EN_NodeProperty_EN_MAXVOLUME, // Tank maximum volume (read only)
    CanOverflow = EN_NodeProperty_EN_CANOVERFLOW, // Tank can overflow (= 1) or not (= 0)
    DemandDeficit = EN_NodeProperty_EN_DEMANDDEFICIT, // Amount that full demand is reduced under PDA (read only)
    NodeInControl = EN_NodeProperty_EN_NODE_INCONTROL, // Is present in any simple or rule-based control (= 1) or not (= 0)
    EmitterFlow = EN_NodeProperty_EN_EMITTERFLOW, // Current emitter flow (read only)
    LeakageFlow = EN_NodeProperty_EN_LEAKAGEFLOW, // Current leakage flow (read only)
    DemandFlow = EN_NodeProperty_EN_DEMANDFLOW, // Current consumer demand delivered (read only)
    FullDemand = EN_NodeProperty_EN_FULLDEMAND, // Current consumer demand requested (read only)
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum NodeType {
    Junction = EN_NodeType_EN_JUNCTION, // Junction node
    Reservoir = EN_NodeType_EN_RESERVOIR, // Reservoir node
    Tank = EN_NodeType_EN_TANK, // Storage tank node
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum SourceType {
    Concen = EN_SourceType_EN_CONCEN, // Sets the concentration of external inflow entering a node
    Mass = EN_SourceType_EN_MASS, // Injects a given mass/minute into a node
    Setpoint = EN_SourceType_EN_SETPOINT, // Sets the concentration leaving a node to a given value
    FlowPaced = EN_SourceType_EN_FLOWPACED, // Adds a given value to the concentration leaving a node
}}
