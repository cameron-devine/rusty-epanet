use crate::bindings::*;
use enum_primitive::*;
#[cfg(test)]
use strum_macros::EnumIter;

/// Max ID Size
pub const MAX_ID_SIZE: EN_SizeLimits = EN_SizeLimits_EN_MAXID;
/// Max message size
pub const MAX_MSG_SIZE: EN_SizeLimits = EN_SizeLimits_EN_MAXMSG;

/// Max project title size. Taken from the EPANET C API source code.
pub const MAX_TITLE_SIZE: EN_SizeLimits = 79;

/// Quality of life struct used as the return object for [`time_to_next_event`] API
pub struct Event {
    pub event_type: TimestepEvent,
    pub duration: u64,
    pub element_index: i32,
}

pub struct QualityAnalysisInfo {
    pub quality_type: QualityType,
    pub chem_name: String,
    pub chem_units: String,
    pub trace_node_index: i32,
}

pub struct DemandModelInfo {
    pub demand_type: DemandModel,
    pub pressure_min: f64,
    pub pressure_required: f64,
    pub pressure_exponent: f64,
}

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
pub enum CurveType {
    VolumeCurve = EN_CurveType_EN_VOLUME_CURVE, // Tank volume v. depth curve
    PumpCurve = EN_CurveType_EN_PUMP_CURVE, // Pump head v. flow curve
    EfficCurve = EN_CurveType_EN_EFFIC_CURVE, // Pump efficiency v. flow curve
    HLossCurve = EN_CurveType_EN_HLOSS_CURVE, // Valve head loss v. flow curve
    GenericCurve = EN_CurveType_EN_GENERIC_CURVE, // Generic curve
    ValveCurve = EN_CurveType_EN_VALVE_CURVE, // Valve loss coeff. v. frac. open
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(EnumIter))]
#[repr(u32)]
pub enum TimeParameter {
    Duration = EN_TimeParameter_EN_DURATION, // Total simulation duration
    HydStep = EN_TimeParameter_EN_HYDSTEP, // Hydraulic time step
    QualStep = EN_TimeParameter_EN_QUALSTEP, // Water quality time step
    PatternStep = EN_TimeParameter_EN_PATTERNSTEP, // Time pattern period
    PatternStart = EN_TimeParameter_EN_PATTERNSTART, // Time when time patterns begin
    ReportStep = EN_TimeParameter_EN_REPORTSTEP, // Reporting time step
    ReportStart = EN_TimeParameter_EN_REPORTSTART, // Time when reporting starts
    RuleStep = EN_TimeParameter_EN_RULESTEP, // Rule-based control evaluation time step
    Statistic = EN_TimeParameter_EN_STATISTIC, // Reporting statistic code
    Periods = EN_TimeParameter_EN_PERIODS, // Number of reporting time periods (read only)
    StartTime = EN_TimeParameter_EN_STARTTIME, // Simulation starting time of day
    HTime = EN_TimeParameter_EN_HTIME, // Elapsed time of current hydraulic solution (read only)
    QTime = EN_TimeParameter_EN_QTIME, // Elapsed time of current quality solution (read only)
    HaltFlag = EN_TimeParameter_EN_HALTFLAG, // Flag indicating if the simulation was halted (read only)
    NextEvent = EN_TimeParameter_EN_NEXTEVENT, // Shortest time until a tank becomes empty or full (read only)
    NextEventTank = EN_TimeParameter_EN_NEXTEVENTTANK, // Index of tank with shortest time to become empty or full (read only)
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum TimestepEvent {
    StepReport = EN_TimestepEvent_EN_STEP_REPORT, // Report all time series points
    StepHyd = EN_TimestepEvent_EN_STEP_HYD, // Hydraulic step
    StepWq = EN_TimestepEvent_EN_STEP_WQ, // Water quality step
    StepTankEvent = EN_TimestepEvent_EN_STEP_TANKEVENT, // Tank event step
    StepControlEvent = EN_TimestepEvent_EN_STEP_CONTROLEVENT, // Control event step
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum AnalysisStatistic {
    Iterations = EN_AnalysisStatistic_EN_ITERATIONS, // Number of hydraulic iterations taken
    RelativeError = EN_AnalysisStatistic_EN_RELATIVEERROR, // Sum of link flow changes / sum of link flows
    MaxHeadError = EN_AnalysisStatistic_EN_MAXHEADERROR, // Largest head loss error for links
    MaxFlowChange = EN_AnalysisStatistic_EN_MAXFLOWCHANGE, // Largest flow change in links
    MassBalance = EN_AnalysisStatistic_EN_MASSBALANCE, // Cumulative water quality mass balance ratio
    DeficientNodes = EN_AnalysisStatistic_EN_DEFICIENTNODES, // Number of pressure deficient nodes
    DemandReduction = EN_AnalysisStatistic_EN_DEMANDREDUCTION, // % demand reduction at pressure deficient nodes
    LeakageLoss = EN_AnalysisStatistic_EN_LEAKAGELOSS, // % flow lost to system leakage
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum ObjectType {
    Node = EN_ObjectType_EN_NODE, // Nodes
    Link = EN_ObjectType_EN_LINK, // Links
    TimePattern = EN_ObjectType_EN_TIMEPAT, // Time patterns
    Curve = EN_ObjectType_EN_CURVE, // Data curves
    Control = EN_ObjectType_EN_CONTROL, // Simple controls
    Rule = EN_ObjectType_EN_RULE, // Control rules
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum CountType {
    NodeCount = EN_CountType_EN_NODECOUNT, // Number of nodes (junctions + tanks + reservoirs)
    TankCount = EN_CountType_EN_TANKCOUNT, // Number of tanks and reservoirs
    LinkCount = EN_CountType_EN_LINKCOUNT, // Number of links (pipes + pumps + valves)
    PatternCount = EN_CountType_EN_PATCOUNT, // Number of time patterns
    CurveCount = EN_CountType_EN_CURVECOUNT, // Number of data curves
    ControlCount = EN_CountType_EN_CONTROLCOUNT, // Number of simple controls
    RuleCount = EN_CountType_EN_RULECOUNT, // Number of rule-based controls
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
pub enum PumpStateType {
    PumpXHead = EN_PumpStateType_EN_PUMP_XHEAD, // Pump closed - cannot supply head
    PumpClosed = EN_PumpStateType_EN_PUMP_CLOSED, // Pump closed
    PumpOpen = EN_PumpStateType_EN_PUMP_OPEN, // Pump open
    PumpXFlow = EN_PumpStateType_EN_PUMP_XFLOW, // Pump open - cannot supply flow
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum QualityType {
    None = EN_QualityType_EN_NONE, // No quality analysis
    Chem = EN_QualityType_EN_CHEM, // Chemical fate and transport
    Age = EN_QualityType_EN_AGE, // Water age analysis
    Trace = EN_QualityType_EN_TRACE, // Source tracing analysis
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

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum HeadLossType {
    HazenWilliams = EN_HeadLossType_EN_HW, // Hazen-Williams
    DarcyWeisbach = EN_HeadLossType_EN_DW, // Darcy-Weisbach
    ChezyManning = EN_HeadLossType_EN_CM, // Chezy-Manning
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum FlowUnits {
    Cfs = EN_FlowUnits_EN_CFS, // Cubic feet per second
    Gpm = EN_FlowUnits_EN_GPM, // Gallons per minute
    Mgd = EN_FlowUnits_EN_MGD, // Million gallons per day
    Imgd = EN_FlowUnits_EN_IMGD, // Imperial million gallons per day
    Afd = EN_FlowUnits_EN_AFD, // Acre-feet per day
    Lps = EN_FlowUnits_EN_LPS, // Liters per second
    Lpm = EN_FlowUnits_EN_LPM, // Liters per minute
    Mld = EN_FlowUnits_EN_MLD, // Million liters per day
    Cmh = EN_FlowUnits_EN_CMH, // Cubic meters per hour
    Cmd = EN_FlowUnits_EN_CMD, // Cubic meters per day
    Cms = EN_FlowUnits_EN_CMS, // Cubic meters per second
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum PressUnits {
    Psi = EN_PressUnits_EN_PSI, // Pounds per square inch
    Kpa = EN_PressUnits_EN_KPA, // Kilopascals
    Meters = EN_PressUnits_EN_METERS, // Meters
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum DemandModel {
    Dda = EN_DemandModel_EN_DDA, // Demand driven analysis
    Pda = EN_DemandModel_EN_PDA, // Pressure driven analysis
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(EnumIter))]
#[repr(u32)]
pub enum Option {
    Trials = EN_Option_EN_TRIALS, // Maximum trials allowed for hydraulic convergence
    Accuracy = EN_Option_EN_ACCURACY, // Total normalized flow change for hydraulic convergence
    Tolerance = EN_Option_EN_TOLERANCE, // Water quality tolerance
    EmitExpon = EN_Option_EN_EMITEXPON, // Exponent in emitter discharge formula
    DemandMult = EN_Option_EN_DEMANDMULT, // Global demand multiplier
    HeadError = EN_Option_EN_HEADERROR, // Maximum head loss error for hydraulic convergence
    FlowChange = EN_Option_EN_FLOWCHANGE, // Maximum flow change for hydraulic convergence
    HeadLossForm = EN_Option_EN_HEADLOSSFORM, // Head loss formula
    GlobalEffic = EN_Option_EN_GLOBALEFFIC, // Global pump efficiency (percent)
    GlobalPrice = EN_Option_EN_GLOBALPRICE, // Global energy price per KWH
    GlobalPattern = EN_Option_EN_GLOBALPATTERN, // Index of a global energy price pattern
    DemandCharge = EN_Option_EN_DEMANDCHARGE, // Energy charge per max. KW usage
    SpGravity = EN_Option_EN_SP_GRAVITY, // Specific gravity
    SpViscos = EN_Option_EN_SP_VISCOS, // Specific viscosity (relative to water at 20 deg C)
    Unbalanced = EN_Option_EN_UNBALANCED, // Extra trials allowed if hydraulics don't converge
    CheckFreq = EN_Option_EN_CHECKFREQ, // Frequency of hydraulic status checks
    MaxCheck = EN_Option_EN_MAXCHECK, // Maximum trials for status checking
    DampLimit = EN_Option_EN_DAMPLIMIT, // Accuracy level where solution damping begins
    SpDiffus = EN_Option_EN_SP_DIFFUS, // Specific diffusivity (relative to chlorine at 20 deg C)
    BulkOrder = EN_Option_EN_BULKORDER, // Bulk water reaction order for pipes
    WallOrder = EN_Option_EN_WALLORDER, // Wall reaction order for pipes (either 0 or 1)
    TankOrder = EN_Option_EN_TANKORDER, // Bulk water reaction order for tanks
    ConcenLimit = EN_Option_EN_CONCENLIMIT, // Limiting concentration for growth reactions
    DemandPattern = EN_Option_EN_DEMANDPATTERN, // Name of default demand pattern
    EmitBackflow = EN_Option_EN_EMITBACKFLOW, // 1 if emitters can backflow, 0 if not
    PressUnits = EN_Option_EN_PRESS_UNITS, // Pressure units
    StatusReport = EN_Option_EN_STATUS_REPORT, // Type of status report to produce
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum ControlType {
    LowLevel = EN_ControlType_EN_LOWLEVEL, // Act when pressure or tank level drops below a setpoint
    HiLevel = EN_ControlType_EN_HILEVEL, // Act when pressure or tank level rises above a setpoint
    Timer = EN_ControlType_EN_TIMER, // Act at a prescribed elapsed amount of time
    TimeOfDay = EN_ControlType_EN_TIMEOFDAY, // Act at a particular time of day
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum StatisticType {
    Series = EN_StatisticType_EN_SERIES, // Report all time series points
    Average = EN_StatisticType_EN_AVERAGE, // Report average value over simulation period
    Minimum = EN_StatisticType_EN_MINIMUM, // Report minimum value over simulation period
    Maximum = EN_StatisticType_EN_MAXIMUM, // Report maximum value over simulation period
    Range = EN_StatisticType_EN_RANGE, // Report maximum - minimum over simulation period
}}

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
pub enum InitHydOption {
    NoSave = EN_InitHydOption_EN_NOSAVE, // Don't save hydraulics; don't re-initialize flows
    Save = EN_InitHydOption_EN_SAVE, // Save hydraulics to file, don't re-initialize flows
    InitFlow = EN_InitHydOption_EN_INITFLOW, // Don't save hydraulics; re-initialize flows
    SaveAndInit = EN_InitHydOption_EN_SAVE_AND_INIT, // Save hydraulics; re-initialize flows
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
pub enum ActionCodeType {
    Unconditional = EN_ActionCodeType_EN_UNCONDITIONAL, // Delete all controls and connecting links
    Conditional = EN_ActionCodeType_EN_CONDITIONAL, // Cancel object deletion if it appears in controls or has connecting links
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum StatusReport {
    NoReport = EN_StatusReport_EN_NO_REPORT, // No status reporting
    NormalReport = EN_StatusReport_EN_NORMAL_REPORT, // Normal level of status reporting
    FullReport = EN_StatusReport_EN_FULL_REPORT, // Full level of status reporting
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum RuleObject {
    Node = EN_RuleObject_EN_R_NODE, // Clause refers to a node
    Link = EN_RuleObject_EN_R_LINK, // Clause refers to a link
    System = EN_RuleObject_EN_R_SYSTEM, // Clause refers to a system parameter (e.g., time)
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum RuleVariable {
    Demand = EN_RuleVariable_EN_R_DEMAND, // Nodal demand
    Head = EN_RuleVariable_EN_R_HEAD, // Nodal hydraulic head
    Grade = EN_RuleVariable_EN_R_GRADE, // Nodal hydraulic grade
    Level = EN_RuleVariable_EN_R_LEVEL, // Tank water level
    Pressure = EN_RuleVariable_EN_R_PRESSURE, // Nodal pressure
    Flow = EN_RuleVariable_EN_R_FLOW, // Link flow rate
    Status = EN_RuleVariable_EN_R_STATUS, // Link status
    Setting = EN_RuleVariable_EN_R_SETTING, // Link setting
    Power = EN_RuleVariable_EN_R_POWER, // Pump power output
    Time = EN_RuleVariable_EN_R_TIME, // Elapsed simulation time
    ClockTime = EN_RuleVariable_EN_R_CLOCKTIME, // Time of day
    FillTime = EN_RuleVariable_EN_R_FILLTIME, // Time to fill a tank
    DrainTime = EN_RuleVariable_EN_R_DRAINTIME, // Time to drain a tank
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum RuleOperator {
    Eq = EN_RuleOperator_EN_R_EQ, // Equal to
    Ne = EN_RuleOperator_EN_R_NE, // Not equal
    Le = EN_RuleOperator_EN_R_LE, // Less than or equal to
    Ge = EN_RuleOperator_EN_R_GE, // Greater than or equal to
    Lt = EN_RuleOperator_EN_R_LT, // Less than
    Gt = EN_RuleOperator_EN_R_GT, // Greater than
    Is = EN_RuleOperator_EN_R_IS, // Is equal to
    Not = EN_RuleOperator_EN_R_NOT, // Is not equal to
    Below = EN_RuleOperator_EN_R_BELOW, // Is below
    Above = EN_RuleOperator_EN_R_ABOVE, // Is above
}}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum RuleStatus {
    IsOpen = EN_RuleStatus_EN_R_IS_OPEN, // Link is open
    IsClosed = EN_RuleStatus_EN_R_IS_CLOSED, // Link is closed
    IsActive = EN_RuleStatus_EN_R_IS_ACTIVE, // Control valve is active
}}
