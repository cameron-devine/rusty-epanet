use crate::bindings::*;
use enum_primitive::*;

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum LinkProperty {
    Diameter = EN_LinkProperty_EN_DIAMETER, // Pipe/valve diameter
    Length = EN_LinkProperty_EN_LENGTH, // Pipe length
    Roughness = EN_LinkProperty_EN_ROUGHNESS, // Pipe roughness coefficient
    MinorLoss = EN_LinkProperty_EN_MINORLOSS, // Pipe/valve minor loss coefficient
    InitStatus = EN_LinkProperty_EN_INITSTATUS, // Initial status
    InitSetting = EN_LinkProperty_EN_INITSETTING, // Initial pump speed or valve setting
    KBulk = EN_LinkProperty_EN_KBULK, // Bulk chemical reaction coefficient
    KWall = EN_LinkProperty_EN_KWALL, // Pipe wall chemical reaction coefficient
    Flow = EN_LinkProperty_EN_FLOW, // Current computed flow rate (read only)
    Velocity = EN_LinkProperty_EN_VELOCITY, // Current computed flow velocity (read only)
    HeadLoss = EN_LinkProperty_EN_HEADLOSS, // Current computed head loss (read only)
    Status = EN_LinkProperty_EN_STATUS, // Current link status
    Setting = EN_LinkProperty_EN_SETTING, // Current link setting
    Energy = EN_LinkProperty_EN_ENERGY, // Current computed pump energy usage (read only)
    LinkQual = EN_LinkProperty_EN_LINKQUAL, // Current computed link quality (read only)
    LinkPattern = EN_LinkProperty_EN_LINKPATTERN, // Pump speed time pattern index
    PumpState = EN_LinkProperty_EN_PUMP_STATE, // Current computed pump state (read only)
    PumpEffic = EN_LinkProperty_EN_PUMP_EFFIC, // Current computed pump efficiency (read only)
    PumpPower = EN_LinkProperty_EN_PUMP_POWER, // Pump constant power rating
    PumpHCurve = EN_LinkProperty_EN_PUMP_HCURVE, // Pump head v. flow curve index
    PumpECurve = EN_LinkProperty_EN_PUMP_ECURVE, // Pump efficiency v. flow curve index
    PumpECost = EN_LinkProperty_EN_PUMP_ECOST, // Pump average energy price
    PumpEPat = EN_LinkProperty_EN_PUMP_EPAT, // Pump energy price time pattern index
    LinkInControl = EN_LinkProperty_EN_LINK_INCONTROL, // Is present in any simple or rule-based control (= 1) or not (= 0)
    GPVCurve = EN_LinkProperty_EN_GPV_CURVE, // GPV head loss v. flow curve index
    PCVCurve = EN_LinkProperty_EN_PCV_CURVE, // PCV loss coeff. curve index
    LeakArea = EN_LinkProperty_EN_LEAK_AREA, // Pipe leak area (sq mm per 100 length units)
    LeakExpan = EN_LinkProperty_EN_LEAK_EXPAN, // Leak expansion rate (sq mm per unit of pressure head)
    LinkLeakage = EN_LinkProperty_EN_LINK_LEAKAGE, // Current leakage rate (read only)
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum LinkType {
    CvPipe = EN_LinkType_EN_CVPIPE, // Pipe with check valve
    Pipe = EN_LinkType_EN_PIPE, // Pipe
    Pump = EN_LinkType_EN_PUMP, // Pump
    Prv = EN_LinkType_EN_PRV, // Pressure reducing valve
    Psv = EN_LinkType_EN_PSV, // Pressure sustaining valve
    Pbv = EN_LinkType_EN_PBV, // Pressure breaker valve
    Fcv = EN_LinkType_EN_FCV, // Flow control valve
    Tcv = EN_LinkType_EN_TCV, // Throttle control valve
    Gpv = EN_LinkType_EN_GPV, // General purpose valve
    Pcv = EN_LinkType_EN_PCV, // Positional control valve
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum LinkStatusType {
    Closed = EN_LinkStatusType_EN_CLOSED, // Link is closed
    Open = EN_LinkStatusType_EN_OPEN, // Link is open
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum PumpType {
    ConstHp = EN_PumpType_EN_CONST_HP, // Constant horsepower
    PowerFunc = EN_PumpType_EN_POWER_FUNC, // Power function
    Custom = EN_PumpType_EN_CUSTOM, // User-defined custom curve
    NoCurve = EN_PumpType_EN_NOCURVE, // No curve
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum PumpStateType {
    PumpXHead = EN_PumpStateType_EN_PUMP_XHEAD, // Pump closed - cannot supply head
    PumpClosed = EN_PumpStateType_EN_PUMP_CLOSED, // Pump closed
    PumpOpen = EN_PumpStateType_EN_PUMP_OPEN, // Pump open
    PumpXFlow = EN_PumpStateType_EN_PUMP_XFLOW, // Pump open - cannot supply flow
}}
