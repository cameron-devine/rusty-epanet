//! Analysis option enumerations: [`FlowUnits`], [`HeadLossType`], [`TimeParameter`],
//! [`Option`], [`QualityType`], and related types.
use crate::bindings::*;
use num_derive::FromPrimitive;
#[cfg(test)]
use strum_macros::EnumIter;

/// Quality of life struct used as the return object for [`time_to_next_event`] API
pub struct Event {
    pub event_type: TimestepEvent,
    pub duration: u64,
    pub element_index: i32,
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum AnalysisStatistic {
    Iterations = EN_AnalysisStatistic_EN_ITERATIONS as i32, // Number of hydraulic iterations taken
    RelativeError = EN_AnalysisStatistic_EN_RELATIVEERROR as i32, // Sum of link flow changes / sum of link flows
    MaxHeadError = EN_AnalysisStatistic_EN_MAXHEADERROR as i32, // Largest head loss error for links
    MaxFlowChange = EN_AnalysisStatistic_EN_MAXFLOWCHANGE as i32, // Largest flow change in links
    MassBalance = EN_AnalysisStatistic_EN_MASSBALANCE as i32, // Cumulative water quality mass balance ratio
    DeficientNodes = EN_AnalysisStatistic_EN_DEFICIENTNODES as i32, // Number of pressure deficient nodes
    DemandReduction = EN_AnalysisStatistic_EN_DEMANDREDUCTION as i32, // % demand reduction at pressure deficient nodes
    LeakageLoss = EN_AnalysisStatistic_EN_LEAKAGELOSS as i32, // % flow lost to system leakage
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum FlowUnits {
    Cfs = EN_FlowUnits_EN_CFS as i32, // Cubic feet per second
    Gpm = EN_FlowUnits_EN_GPM as i32, // Gallons per minute
    Mgd = EN_FlowUnits_EN_MGD as i32, // Million gallons per day
    Imgd = EN_FlowUnits_EN_IMGD as i32, // Imperial million gallons per day
    Afd = EN_FlowUnits_EN_AFD as i32, // Acre-feet per day
    Lps = EN_FlowUnits_EN_LPS as i32, // Liters per second
    Lpm = EN_FlowUnits_EN_LPM as i32, // Liters per minute
    Mld = EN_FlowUnits_EN_MLD as i32, // Million liters per day
    Cmh = EN_FlowUnits_EN_CMH as i32, // Cubic meters per hour
    Cmd = EN_FlowUnits_EN_CMD as i32, // Cubic meters per day
    Cms = EN_FlowUnits_EN_CMS as i32, // Cubic meters per second
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum HeadLossType {
    HazenWilliams = EN_HeadLossType_EN_HW as i32, // Hazen-Williams
    DarcyWeisbach = EN_HeadLossType_EN_DW as i32, // Darcy-Weisbach
    ChezyManning = EN_HeadLossType_EN_CM as i32, // Chezy-Manning
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum PressUnits {
    Psi = EN_PressUnits_EN_PSI as i32, // Pounds per square inch
    Kpa = EN_PressUnits_EN_KPA as i32, // Kilopascals
    Meters = EN_PressUnits_EN_METERS as i32, // Meters
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[cfg_attr(test, derive(EnumIter))]
#[repr(i32)]
pub enum Option {
    Trials = EN_Option_EN_TRIALS as i32, // Maximum trials allowed for hydraulic convergence
    Accuracy = EN_Option_EN_ACCURACY as i32, // Total normalized flow change for hydraulic convergence
    Tolerance = EN_Option_EN_TOLERANCE as i32, // Water quality tolerance
    EmitExpon = EN_Option_EN_EMITEXPON as i32, // Exponent in emitter discharge formula
    DemandMult = EN_Option_EN_DEMANDMULT as i32, // Global demand multiplier
    HeadError = EN_Option_EN_HEADERROR as i32, // Maximum head loss error for hydraulic convergence
    FlowChange = EN_Option_EN_FLOWCHANGE as i32, // Maximum flow change for hydraulic convergence
    HeadLossForm = EN_Option_EN_HEADLOSSFORM as i32, // Head loss formula
    GlobalEffic = EN_Option_EN_GLOBALEFFIC as i32, // Global pump efficiency (percent)
    GlobalPrice = EN_Option_EN_GLOBALPRICE as i32, // Global energy price per KWH
    GlobalPattern = EN_Option_EN_GLOBALPATTERN as i32, // Index of a global energy price pattern
    DemandCharge = EN_Option_EN_DEMANDCHARGE as i32, // Energy charge per max. KW usage
    SpGravity = EN_Option_EN_SP_GRAVITY as i32, // Specific gravity
    SpViscos = EN_Option_EN_SP_VISCOS as i32, // Specific viscosity (relative to water at 20 deg C)
    Unbalanced = EN_Option_EN_UNBALANCED as i32, // Extra trials allowed if hydraulics don't converge
    CheckFreq = EN_Option_EN_CHECKFREQ as i32, // Frequency of hydraulic status checks
    MaxCheck = EN_Option_EN_MAXCHECK as i32, // Maximum trials for status checking
    DampLimit = EN_Option_EN_DAMPLIMIT as i32, // Accuracy level where solution damping begins
    SpDiffus = EN_Option_EN_SP_DIFFUS as i32, // Specific diffusivity (relative to chlorine at 20 deg C)
    BulkOrder = EN_Option_EN_BULKORDER as i32, // Bulk water reaction order for pipes
    WallOrder = EN_Option_EN_WALLORDER as i32, // Wall reaction order for pipes (either 0 or 1)
    TankOrder = EN_Option_EN_TANKORDER as i32, // Bulk water reaction order for tanks
    ConcenLimit = EN_Option_EN_CONCENLIMIT as i32, // Limiting concentration for growth reactions
    DemandPattern = EN_Option_EN_DEMANDPATTERN as i32, // Name of default demand pattern
    EmitBackflow = EN_Option_EN_EMITBACKFLOW as i32, // 1 if emitters can backflow, 0 if not
    PressUnits = EN_Option_EN_PRESS_UNITS as i32, // Pressure units
    StatusReport = EN_Option_EN_STATUS_REPORT as i32, // Type of status report to produce
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum QualityType {
    None = EN_QualityType_EN_NONE as i32, // No quality analysis
    Chem = EN_QualityType_EN_CHEM as i32, // Chemical fate and transport
    Age = EN_QualityType_EN_AGE as i32, // Water age analysis
    Trace = EN_QualityType_EN_TRACE as i32, // Source tracing analysis
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum StatisticType {
    Series = EN_StatisticType_EN_SERIES as i32, // Report all time series points
    Average = EN_StatisticType_EN_AVERAGE as i32, // Report average value over simulation period
    Minimum = EN_StatisticType_EN_MINIMUM as i32, // Report minimum value over simulation period
    Maximum = EN_StatisticType_EN_MAXIMUM as i32, // Report maximum value over simulation period
    Range = EN_StatisticType_EN_RANGE as i32, // Report maximum - minimum over simulation period
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum StatusReport {
    NoReport = EN_StatusReport_EN_NO_REPORT as i32, // No status reporting
    NormalReport = EN_StatusReport_EN_NORMAL_REPORT as i32, // Normal level of status reporting
    FullReport = EN_StatusReport_EN_FULL_REPORT as i32, // Full level of status reporting
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[cfg_attr(test, derive(EnumIter))]
#[repr(i32)]
pub enum TimeParameter {
    Duration = EN_TimeParameter_EN_DURATION as i32, // Total simulation duration
    HydStep = EN_TimeParameter_EN_HYDSTEP as i32, // Hydraulic time step
    QualStep = EN_TimeParameter_EN_QUALSTEP as i32, // Water quality time step
    PatternStep = EN_TimeParameter_EN_PATTERNSTEP as i32, // Time pattern period
    PatternStart = EN_TimeParameter_EN_PATTERNSTART as i32, // Time when time patterns begin
    ReportStep = EN_TimeParameter_EN_REPORTSTEP as i32, // Reporting time step
    ReportStart = EN_TimeParameter_EN_REPORTSTART as i32, // Time when reporting starts
    RuleStep = EN_TimeParameter_EN_RULESTEP as i32, // Rule-based control evaluation time step
    Statistic = EN_TimeParameter_EN_STATISTIC as i32, // Reporting statistic code
    Periods = EN_TimeParameter_EN_PERIODS as i32, // Number of reporting time periods (read only)
    StartTime = EN_TimeParameter_EN_STARTTIME as i32, // Simulation starting time of day
    HTime = EN_TimeParameter_EN_HTIME as i32, // Elapsed time of current hydraulic solution (read only)
    QTime = EN_TimeParameter_EN_QTIME as i32, // Elapsed time of current quality solution (read only)
    HaltFlag = EN_TimeParameter_EN_HALTFLAG as i32, // Flag indicating if the simulation was halted (read only)
    NextEvent = EN_TimeParameter_EN_NEXTEVENT as i32, // Shortest time until a tank becomes empty or full (read only)
    NextEventTank = EN_TimeParameter_EN_NEXTEVENTTANK as i32, // Index of tank with shortest time to become empty or full (read only)
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum TimestepEvent {
    StepReport = EN_TimestepEvent_EN_STEP_REPORT as i32, // Report all time series points
    StepHyd = EN_TimestepEvent_EN_STEP_HYD as i32, // Hydraulic step
    StepWq = EN_TimestepEvent_EN_STEP_WQ as i32, // Water quality step
    StepTankEvent = EN_TimestepEvent_EN_STEP_TANKEVENT as i32, // Tank event step
    StepControlEvent = EN_TimestepEvent_EN_STEP_CONTROLEVENT as i32, // Control event step
}

pub struct QualityAnalysisInfo {
    pub quality_type: QualityType,
    pub chem_name: String,
    pub chem_units: String,
    pub trace_node_index: i32,
}
