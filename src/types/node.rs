use crate::bindings::*;
use crate::epanet_error::*;
use crate::EPANET;
use num_derive::FromPrimitive;
use crate::types::ActionCodeType;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum MixingModel {
    Mix1 = EN_MixingModel_EN_MIX1 as i32, // Complete mix model
    Mix2 = EN_MixingModel_EN_MIX2 as i32, // 2-compartment model
    Fifo = EN_MixingModel_EN_FIFO as i32, // First in, first out model
    Lifo = EN_MixingModel_EN_LIFO as i32, // Last in, first out model
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum NodeProperty {
    Elevation = EN_NodeProperty_EN_ELEVATION as i32, // Elevation
    BaseDemand = EN_NodeProperty_EN_BASEDEMAND as i32, // Primary demand baseline value
    Pattern = EN_NodeProperty_EN_PATTERN as i32, // Primary demand time pattern index
    Emitter = EN_NodeProperty_EN_EMITTER as i32, // Emitter flow coefficient
    InitQual = EN_NodeProperty_EN_INITQUAL as i32, // Initial quality
    SourceQual = EN_NodeProperty_EN_SOURCEQUAL as i32, // Quality source strength
    SourcePat = EN_NodeProperty_EN_SOURCEPAT as i32, // Quality source pattern index
    SourceType = EN_NodeProperty_EN_SOURCETYPE as i32, // Quality source type
    TankLevel = EN_NodeProperty_EN_TANKLEVEL as i32, // Current computed tank water level (read only)
    Demand = EN_NodeProperty_EN_DEMAND as i32, // Current computed demand (read only)
    Head = EN_NodeProperty_EN_HEAD as i32, // Current computed hydraulic head (read only)
    Pressure = EN_NodeProperty_EN_PRESSURE as i32, // Current computed pressure (read only)
    Quality = EN_NodeProperty_EN_QUALITY as i32, // Current computed quality (read only)
    SourceMass = EN_NodeProperty_EN_SOURCEMASS as i32, // Current computed quality source mass inflow (read only)
    InitVolume = EN_NodeProperty_EN_INITVOLUME as i32, // Tank initial volume (read only)
    MixModel = EN_NodeProperty_EN_MIXMODEL as i32, // Tank mixing model
    MixZoneVol = EN_NodeProperty_EN_MIXZONEVOL as i32, // Tank mixing zone volume (read only)
    TankDiam = EN_NodeProperty_EN_TANKDIAM as i32, // Tank diameter
    MinVolume = EN_NodeProperty_EN_MINVOLUME as i32, // Tank minimum volume
    VolCurve = EN_NodeProperty_EN_VOLCURVE as i32, // Tank volume curve index
    MinLevel = EN_NodeProperty_EN_MINLEVEL as i32, // Tank minimum level
    MaxLevel = EN_NodeProperty_EN_MAXLEVEL as i32, // Tank maximum level
    MixFraction = EN_NodeProperty_EN_MIXFRACTION as i32, // Tank mixing fraction
    TankKBulk = EN_NodeProperty_EN_TANK_KBULK as i32, // Tank bulk decay coefficient
    TankVolume = EN_NodeProperty_EN_TANKVOLUME as i32, // Current computed tank volume (read only)
    MaxVolume = EN_NodeProperty_EN_MAXVOLUME as i32, // Tank maximum volume (read only)
    CanOverflow = EN_NodeProperty_EN_CANOVERFLOW as i32, // Tank can overflow (= 1) or not (= 0)
    DemandDeficit = EN_NodeProperty_EN_DEMANDDEFICIT as i32, // Amount that full demand is reduced under PDA (read only)
    NodeInControl = EN_NodeProperty_EN_NODE_INCONTROL as i32, // Is present in any simple or rule-based control (= 1) or not (= 0)
    EmitterFlow = EN_NodeProperty_EN_EMITTERFLOW as i32, // Current emitter flow (read only)
    LeakageFlow = EN_NodeProperty_EN_LEAKAGEFLOW as i32, // Current leakage flow (read only)
    DemandFlow = EN_NodeProperty_EN_DEMANDFLOW as i32, // Current consumer demand delivered (read only)
    FullDemand = EN_NodeProperty_EN_FULLDEMAND as i32, // Current consumer demand requested (read only)
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum NodeType {
    Junction = EN_NodeType_EN_JUNCTION as i32, // Junction node
    Reservoir = EN_NodeType_EN_RESERVOIR as i32, // Reservoir node
    Tank = EN_NodeType_EN_TANK as i32, // Storage tank node
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum SourceType {
    Concen = EN_SourceType_EN_CONCEN as i32, // Sets the concentration of external inflow entering a node
    Mass = EN_SourceType_EN_MASS as i32, // Injects a given mass/minute into a node
    Setpoint = EN_SourceType_EN_SETPOINT as i32, // Sets the concentration leaving a node to a given value
    FlowPaced = EN_SourceType_EN_FLOWPACED as i32, // Adds a given value to the concentration leaving a node
}

/// A wrapper around an EPANET node index, type, and id.
///
/// ```ignore
/// use epanet::types::node::{NodeType, NodeKind, Node};
/// # fn demo(ph: &epanet::EPANET) -> epanet::epanet_error::Result<()> {
/// // Add a junction and access typed properties
/// let node = Node::new(ph, "J1", NodeType::Junction)?;
/// match node.kind() {
///     NodeKind::Junction(j) => {
///         let _demand = j.base_demand()?;
///     }
///     _ => unreachable!(),
/// }
/// # Ok(()) }
/// ```
pub struct Node<'a> {
    pub(crate) project: &'a EPANET,
    pub(crate) index: i32,
    pub id: String,
    pub kind: NodeKind,
}

pub enum NodeKind {
    Junction(JunctionData),
    Tank(TankData),
    Reservoir(ReservoirData)
}

pub struct JunctionData {
    pub elevation: f64,
    pub demand: f64,
    pub demand_pattern: String
}

pub struct TankData {
    pub elevation: f64,
    pub init_level: f64,
    pub min_level: f64,
    pub max_level: f64,
    pub diameter: f64,
    pub min_volume: f64,
    pub volume_curve: String,
}

pub struct ReservoirData {
    pub elevation: f64
}

impl<'a> Node<'a> {
    /// Creates a new junction node in the EPANET model.
    pub fn new_junction(
        project: &'a EPANET,
        id: &str,
        elevation: f64,
        demand: f64,
        demand_pattern: &str,
    ) -> Result<Self> {
        let index = project.add_node(id, NodeType::Junction)?;
        project.set_junction_data(index, elevation, demand, demand_pattern)?;

        Ok(Node {
            project,
            index,
            id: id.to_string(),
            kind: NodeKind::Junction(JunctionData {
                elevation,
                demand,
                demand_pattern: demand_pattern.to_string(),
            }),
        })
    }

    /// Creates a new tank node in the EPANET model.
    pub fn new_tank(
        project: &'a EPANET,
        id: &str,
        elevation: f64,
        init_level: f64,
        min_level: f64,
        max_level: f64,
        diameter: f64,
        min_volume: f64,
        volume_curve: &str,
    ) -> Result<Self> {
        let index = project.add_node(id, NodeType::Tank)?;
        project.set_tank_data(
            index,
            elevation,
            init_level,
            min_level,
            max_level,
            diameter,
            min_volume,
            volume_curve,
        )?;

        Ok(Node {
            project,
            index,
            id: id.to_string(),
            kind: NodeKind::Tank(TankData {
                elevation,
                init_level,
                min_level,
                max_level,
                diameter,
                min_volume,
                volume_curve: volume_curve.to_string(),
            }),
        })
    }

    /// Creates a new reservoir node in the EPANET model.
    pub fn new_reservoir(project: &'a EPANET, id: &str, elevation: f64) -> Result<Self> {
        let index = project.add_node(id, NodeType::Reservoir)?;
        project.set_node_value(index, NodeProperty::Elevation, elevation)?;

        Ok(Node {
            project,
            index,
            id: id.to_string(),
            kind: NodeKind::Reservoir(ReservoirData { elevation }),
        })
    }

    pub fn update(&self) -> Result<()> {
        // Only update ID if it has changed
        let current_id = self.project.get_node_id(self.index)?;
        if current_id != self.id {
            self.project.set_node_id(self.index, &self.id)?;
        }

        match &self.kind {
            NodeKind::Junction(d) => {
                self.project.set_junction_data(self.index, d.elevation, d.demand, &d.demand_pattern)?;
                Ok(())
            }
            NodeKind::Tank(d) => {
                self.project.set_tank_data(
                    self.index,
                    d.elevation,
                    d.init_level,
                    d.min_level,
                    d.max_level,
                    d.diameter,
                    d.min_volume,
                    &d.volume_curve,
                )
            }
            // todo: Double check if reservoirs have other properties to set
            NodeKind::Reservoir(d) => {
                self.project.set_node_value(self.index, NodeProperty::Elevation, d.elevation)
            }
        }
    }
    pub fn delete(self, action_code: ActionCodeType) -> Result<()> {
        self.project.delete_node(self.index, action_code)
    }
    pub fn pressure(&self) -> Result<f64> {
        self.project.get_node_value(self.index, NodeProperty::Pressure)
    }
    pub fn head(&self) -> Result<f64> {
        self.project.get_node_value(self.index, NodeProperty::Head)
    }
    pub fn demand(&self) -> Result<f64> {
        self.project.get_node_value(self.index, NodeProperty::Demand)
    }  // computed demand, not base
    pub fn quality(&self) -> Result<f64> {
        self.project.get_node_value(self.index, NodeProperty::Quality)
    }

    pub fn as_junction(&self) -> Option<&JunctionData> {
        match &self.kind {
            NodeKind::Junction(d) => Some(d),
            _ => None
        }
    }

    pub fn as_junction_mut(&mut self) -> Option<&mut JunctionData> {
        match &mut self.kind {
            NodeKind::Junction(d) => Some(d),
            _ => None
        }
    }

    pub fn as_tank(&self) -> Option<&TankData> {
        match &self.kind {
            NodeKind::Tank(d) => Some(d),
            _ => None
        }
    }

    pub fn as_tank_mut(&mut self) -> Option<&mut TankData> {
        match &mut self.kind {
            NodeKind::Tank(d) => Some(d),
            _ => None
        }
    }

    pub fn as_reservoir(&self) -> Option<&ReservoirData> {
        match &self.kind {
            NodeKind::Reservoir(d) => Some(d),
            _ => None
        }
    }

    pub fn as_reservoir_mut(&mut self) -> Option<&mut ReservoirData> {
        match &mut self.kind {
            NodeKind::Reservoir(d) => Some(d),
            _ => None
        }
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn node_type(&self) -> NodeType {
        match &self.kind {
            NodeKind::Junction(_) => NodeType::Junction,
            NodeKind::Tank(_) => NodeType::Tank,
            NodeKind::Reservoir(_) => NodeType::Reservoir,
        }
    }

    pub fn is_junction(&self) -> bool {
        matches!(self.kind, NodeKind::Junction(_))
    }

    pub fn is_tank(&self) -> bool {
        matches!(self.kind, NodeKind::Tank(_))
    }

    pub fn is_reservoir(&self) -> bool {
        matches!(self.kind, NodeKind::Reservoir(_))
    }

    /// Get coordinates for this node.
    pub fn coordinates(&self) -> Result<(f64, f64)> {
        self.project.get_coordinates(self.index)
    }

    /// Set coordinates for this node.
    pub fn set_coordinates(&self, x: f64, y: f64) -> Result<()> {
        self.project.set_coordinates(self.index, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::impls::test_utils::fixtures::*;

    #[rstest]
    fn test_node_create_junction(ph_close: crate::EPANET) {
        let node = Node::new_junction(&ph_close, "J1", 100.0, 50.0, "").unwrap();
        assert_eq!(node.id, "J1");
        assert!(node.is_junction());
        assert!(!node.is_tank());
        assert!(!node.is_reservoir());

        let junction_data = node.as_junction().unwrap();
        assert_eq!(junction_data.elevation, 100.0);
        assert_eq!(junction_data.demand, 50.0);
    }

    #[rstest]
    fn test_node_create_reservoir(ph_close: crate::EPANET) {
        let node = Node::new_reservoir(&ph_close, "R1", 200.0).unwrap();
        assert_eq!(node.id, "R1");
        assert!(node.is_reservoir());
        assert!(!node.is_junction());
        assert!(!node.is_tank());

        let reservoir_data = node.as_reservoir().unwrap();
        assert_eq!(reservoir_data.elevation, 200.0);
    }

    #[rstest]
    fn test_node_get_from_model(ph: crate::EPANET) {
        // Get an existing junction from the test model
        let node = ph.get_node("11").unwrap();
        assert_eq!(node.id, "11");
        assert!(node.is_junction());

        let junction_data = node.as_junction().unwrap();
        assert_eq!(junction_data.elevation, 710.0);
        assert_eq!(junction_data.demand, 150.0);
    }

    #[rstest]
    fn test_node_update(ph_close: crate::EPANET) {
        let mut node = Node::new_junction(&ph_close, "J2", 100.0, 50.0, "").unwrap();

        // Modify data
        if let Some(junction_data) = node.as_junction_mut() {
            junction_data.elevation = 150.0;
            junction_data.demand = 75.0;
        }

        // Update in model
        node.update().unwrap();

        // Verify the changes persisted
        let retrieved = ph_close.get_node("J2").unwrap();
        let retrieved_data = retrieved.as_junction().unwrap();
        assert_eq!(retrieved_data.elevation, 150.0);
        assert_eq!(retrieved_data.demand, 75.0);
    }
}
