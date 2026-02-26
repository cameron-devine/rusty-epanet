use crate::bindings::*;
use num_derive::FromPrimitive;
use crate::EPANET;
use crate::types::ActionCodeType;
use crate::epanet_error::*;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
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
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
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
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum LinkStatusType {
    Closed = EN_LinkStatusType_EN_CLOSED, // Link is closed
    Open = EN_LinkStatusType_EN_OPEN, // Link is open
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum PumpType {
    ConstHp = EN_PumpType_EN_CONST_HP, // Constant horsepower
    PowerFunc = EN_PumpType_EN_POWER_FUNC, // Power function
    Custom = EN_PumpType_EN_CUSTOM, // User-defined custom curve
    NoCurve = EN_PumpType_EN_NOCURVE, // No curve
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum PumpStateType {
    PumpXHead = EN_PumpStateType_EN_PUMP_XHEAD, // Pump closed - cannot supply head
    PumpClosed = EN_PumpStateType_EN_PUMP_CLOSED, // Pump closed
    PumpOpen = EN_PumpStateType_EN_PUMP_OPEN, // Pump open
    PumpXFlow = EN_PumpStateType_EN_PUMP_XFLOW, // Pump open - cannot supply flow
}

/// A link in the EPANET network model.
///
/// This is a snapshot/view of the C engine state. Fields are cached
/// on construction; call `update()` to push changes back, or use
/// the live query methods (flow(), velocity(), etc.) for computed results.
pub struct Link<'a> {
    pub(crate) project: &'a EPANET,
    pub(crate) index: i32,
    pub id: String,
    pub from_node: i32,
    pub to_node: i32,
    pub status: LinkStatusType,
    pub kind: LinkKind,
}

pub enum LinkKind {
    Pipe(PipeData),
    CvPipe(PipeData),          // same fields, different hydraulic behavior
    Pump(PumpData),
    Valve(ValveData),
}

pub struct PipeData {
    pub length: f64,
    pub diameter: f64,
    pub roughness: f64,
    pub minor_loss: f64,
}

pub struct PumpData {
    pub pump_type: PumpType,
    pub power: f64,
    pub speed: f64,
    pub head_curve_index: Option<i32>,
    pub efficiency_curve_index: Option<i32>,
    pub energy_pattern_index: Option<i32>,
    pub energy_cost: f64,
}

pub struct ValveData {
    pub diameter: f64,
    pub setting: f64,           // meaning depends on valve_type
    pub curve_index: Option<i32>, // GPV/PCV only
}

impl<'a> Link<'a> {
    /// Creates a new pipe link in the EPANET model.
    pub fn new_pipe(
        project: &'a EPANET,
        id: &str,
        from_node: &str,
        to_node: &str,
        length: f64,
        diameter: f64,
        roughness: f64,
        minor_loss: f64,
    ) -> Result<Self> {
        let index = project.add_link(id, LinkType::Pipe, from_node, to_node)?;
        project.set_pipe_data(index, length, diameter, roughness, minor_loss)?;

        let from_node_idx = project.get_node_index(from_node)?;
        let to_node_idx = project.get_node_index(to_node)?;

        Ok(Link {
            project,
            index,
            id: id.to_string(),
            from_node: from_node_idx,
            to_node: to_node_idx,
            status: LinkStatusType::Open,
            kind: LinkKind::Pipe(PipeData {
                length,
                diameter,
                roughness,
                minor_loss,
            }),
        })
    }

    /// Creates a new pump link in the EPANET model.
    pub fn new_pump(
        project: &'a EPANET,
        id: &str,
        from_node: &str,
        to_node: &str,
        power: f64,
        speed: f64,
        head_curve_index: Option<i32>,
    ) -> Result<Self> {
        let index = project.add_link(id, LinkType::Pump, from_node, to_node)?;
        project.set_link_value(index, LinkProperty::PumpPower, power)?;
        project.set_link_value(index, LinkProperty::InitSetting, speed)?;

        if let Some(curve_idx) = head_curve_index {
            project.set_head_curve_index(index, curve_idx)?;
        }

        let from_node_idx = project.get_node_index(from_node)?;
        let to_node_idx = project.get_node_index(to_node)?;
        let pump_type = project.get_pump_type(index)?;

        Ok(Link {
            project,
            index,
            id: id.to_string(),
            from_node: from_node_idx,
            to_node: to_node_idx,
            status: LinkStatusType::Open,
            kind: LinkKind::Pump(PumpData {
                pump_type,
                power,
                speed,
                head_curve_index,
                efficiency_curve_index: None,
                energy_pattern_index: None,
                energy_cost: 0.0,
            }),
        })
    }

    /// Creates a new valve link in the EPANET model.
    ///
    /// Note: `link_type` should be one of the valve types (PRV, PSV, PBV, FCV, TCV, GPV, PCV).
    pub fn new_valve(
        project: &'a EPANET,
        id: &str,
        link_type: LinkType,
        from_node: &str,
        to_node: &str,
        diameter: f64,
        setting: f64,
    ) -> Result<Self> {
        let index = project.add_link(id, link_type, from_node, to_node)?;
        project.set_link_value(index, LinkProperty::Diameter, diameter)?;
        project.set_link_value(index, LinkProperty::InitSetting, setting)?;

        let from_node_idx = project.get_node_index(from_node)?;
        let to_node_idx = project.get_node_index(to_node)?;

        Ok(Link {
            project,
            index,
            id: id.to_string(),
            from_node: from_node_idx,
            to_node: to_node_idx,
            status: LinkStatusType::Open,
            kind: LinkKind::Valve(ValveData {
                diameter,
                setting,
                curve_index: None,
            }),
        })
    }

    pub fn index(&self) -> i32 { self.index }

    /// Push cached fields back to the C engine.
    pub fn update(&self) -> Result<()> {
        // Only update ID if it has changed
        let current_id = self.project.get_link_id(self.index)?;
        if current_id != self.id {
            self.project.set_link_id(self.index, &self.id)?;
        }

        self.project.set_link_nodes(self.index, self.from_node, self.to_node)?;

        // type-specific fields
        match &self.kind {
            LinkKind::Pipe(d) | LinkKind::CvPipe(d) => {
                self.project.set_pipe_data(
                    self.index, d.length, d.diameter, d.roughness, d.minor_loss,
                )
            }
            LinkKind::Pump(d) => {
                self.project.set_link_value(self.index, LinkProperty::PumpPower, d.power)?;
                self.project.set_link_value(self.index, LinkProperty::InitSetting, d.speed)?;
                if let Some(ci) = d.head_curve_index {
                    self.project.set_head_curve_index(self.index, ci)?;
                }
                Ok(())
            }
            LinkKind::Valve(d) => {
                self.project.set_link_value(self.index, LinkProperty::Diameter, d.diameter)?;
                self.project.set_link_value(self.index, LinkProperty::InitSetting, d.setting)
            }
        }
    }

    pub fn delete(self, action_code: ActionCodeType) -> Result<()> {
        self.project.delete_link(self.index, action_code)
    }

    // --- Live computed results (read from C engine, not cached) ---

    pub fn flow(&self) -> Result<f64> {
        self.project.get_link_value(self.index, LinkProperty::Flow)
    }

    pub fn velocity(&self) -> Result<f64> {
        self.project.get_link_value(self.index, LinkProperty::Velocity)
    }

    pub fn head_loss(&self) -> Result<f64> {
        self.project.get_link_value(self.index, LinkProperty::HeadLoss)
    }

    pub fn quality(&self) -> Result<f64> {
        self.project.get_link_value(self.index, LinkProperty::LinkQual)
    }

    // --- Convenience type checks ---

    pub fn is_pipe(&self) -> bool {
        matches!(self.kind, LinkKind::Pipe(_) | LinkKind::CvPipe(_))
    }

    pub fn is_pump(&self) -> bool {
        matches!(self.kind, LinkKind::Pump(_))
    }

    pub fn is_valve(&self) -> bool {
        matches!(self.kind, LinkKind::Valve(_))
    }

    /// Get pipe data, if this link is a pipe.
    pub fn as_pipe(&self) -> Option<&PipeData> {
        match &self.kind {
            LinkKind::Pipe(d) | LinkKind::CvPipe(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_pipe_mut(&mut self) -> Option<&mut PipeData> {
        match &mut self.kind {
            LinkKind::Pipe(d) | LinkKind::CvPipe(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_pump(&self) -> Option<&PumpData> {
        match &self.kind { LinkKind::Pump(d) => Some(d), _ => None }
    }

    pub fn as_pump_mut(&mut self) -> Option<&mut PumpData> {
        match &mut self.kind { LinkKind::Pump(d) => Some(d), _ => None }
    }

    pub fn as_valve(&self) -> Option<&ValveData> {
        match &self.kind { LinkKind::Valve(d) => Some(d), _ => None }
    }

    pub fn as_valve_mut(&mut self) -> Option<&mut ValveData> {
        match &mut self.kind { LinkKind::Valve(d) => Some(d), _ => None }
    }

    /// Returns the link type.
    pub fn link_type(&self) -> LinkType {
        match &self.kind {
            LinkKind::Pipe(_) => LinkType::Pipe,
            LinkKind::CvPipe(_) => LinkType::CvPipe,
            LinkKind::Pump(_) => LinkType::Pump,
            // For valves, we can't determine the exact type from LinkKind::Valve
            // So we query the model
            LinkKind::Valve(_) => self.project.get_link_type(self.index).unwrap_or(LinkType::Prv),
        }
    }

    /// Get all vertices for this link.
    pub fn vertices(&self) -> Result<Vec<(f64, f64)>> {
        let count = self.project.get_vertex_count(self.index)?;
        let mut vertices = Vec::with_capacity(count as usize);
        for i in 1..=count {
            vertices.push(self.project.get_vertex(self.index, i)?);
        }
        Ok(vertices)
    }

    /// Set all vertices for this link.
    pub fn set_vertices(&self, vertices: Vec<(f64, f64)>) -> Result<()> {
        self.project.set_vertices(self.index, vertices)
    }

    /// Get the start and end node indices.
    pub fn nodes(&self) -> (i32, i32) {
        (self.from_node, self.to_node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::impls::test_utils::fixtures::*;

    #[rstest]
    fn test_link_create_pipe(ph_close: crate::EPANET) {
        // First create nodes
        let _n1 = crate::types::node::Node::new_junction(&ph_close, "N1", 100.0, 50.0, "").unwrap();
        let _n2 = crate::types::node::Node::new_junction(&ph_close, "N2", 100.0, 50.0, "").unwrap();

        let link = Link::new_pipe(&ph_close, "P1", "N1", "N2", 1000.0, 12.0, 100.0, 0.0).unwrap();
        assert_eq!(link.id, "P1");
        assert!(link.is_pipe());
        assert!(!link.is_pump());
        assert!(!link.is_valve());

        let pipe_data = link.as_pipe().unwrap();
        assert_eq!(pipe_data.length, 1000.0);
        assert_eq!(pipe_data.diameter, 12.0);
        assert_eq!(pipe_data.roughness, 100.0);
        assert_eq!(pipe_data.minor_loss, 0.0);
    }

    #[rstest]
    fn test_link_create_pump(ph_close: crate::EPANET) {
        // Create nodes
        let _n1 = crate::types::node::Node::new_junction(&ph_close, "N1", 100.0, 50.0, "").unwrap();
        let _n2 = crate::types::node::Node::new_junction(&ph_close, "N2", 100.0, 50.0, "").unwrap();

        let link = Link::new_pump(&ph_close, "PMP1", "N1", "N2", 75.0, 1.0, None).unwrap();
        assert_eq!(link.id, "PMP1");
        assert!(link.is_pump());
        assert!(!link.is_pipe());

        let pump_data = link.as_pump().unwrap();
        assert_eq!(pump_data.power, 75.0);
        assert_eq!(pump_data.speed, 1.0);
    }

    #[rstest]
    fn test_link_get_from_model(ph: crate::EPANET) {
        // Get an existing pipe from the test model
        let link = ph.get_link("10").unwrap();
        assert_eq!(link.id, "10");
        assert!(link.is_pipe());

        let pipe_data = link.as_pipe().unwrap();
        assert!(pipe_data.length > 0.0);
        assert!(pipe_data.diameter > 0.0);
    }

    #[rstest]
    fn test_link_update(ph_close: crate::EPANET) {
        // Create nodes
        let _n1 = crate::types::node::Node::new_junction(&ph_close, "N1", 100.0, 50.0, "").unwrap();
        let _n2 = crate::types::node::Node::new_junction(&ph_close, "N2", 100.0, 50.0, "").unwrap();

        let mut link = Link::new_pipe(&ph_close, "P1", "N1", "N2", 1000.0, 12.0, 100.0, 0.0).unwrap();

        // Modify data
        if let Some(pipe_data) = link.as_pipe_mut() {
            pipe_data.length = 2000.0;
            pipe_data.diameter = 14.0;
        }

        // Update in model
        link.update().unwrap();

        // Verify the changes persisted
        let retrieved = ph_close.get_link("P1").unwrap();
        let retrieved_data = retrieved.as_pipe().unwrap();
        assert_eq!(retrieved_data.length, 2000.0);
        assert_eq!(retrieved_data.diameter, 14.0);
    }
}

