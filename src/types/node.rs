use crate::bindings::*;
use crate::epanet_error::*;
use crate::types::ActionCodeType::Unconditional;
use crate::EPANET;
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
    pub(crate) handle: &'a EPANET,
    index: i32,
    id: String,
    node_type: NodeType,
}

impl<'a> Node<'a> {
    /// Creates a new node and wraps it in [`Node`].
    pub fn new(handle: &'a EPANET, id: &str, node_type: NodeType) -> Result<Self> {
        let index = handle.add_node(id, node_type)?;
        Ok(Node {
            handle,
            index,
            id: id.to_string(),
            node_type,
        })
    }

    /// Deletes a [`Node`] from the project
    pub fn delete(self) -> Result<()> {
        self.handle.delete_node(self.index, Unconditional)
    }

    /// Creates a [`Node`] from an existing index.
    pub fn from_index(handle: &'a EPANET, index: i32) -> Result<Self> {
        let id = handle.get_node_id(index)?;
        let node_type = handle.get_node_type(index)?;

        Ok(Node {
            handle,
            index,
            id,
            node_type,
        })
    }

    /// Get the index of the node
    pub fn get_index(&self) -> i32 {
        self.index
    }
    
    /// Get the type of the node
    pub fn get_type(&self) -> NodeType {
        self.node_type
    }

    /// Gets the node id
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    /// Sets the node id
    pub fn set_id(&mut self, id: &str) -> Result<()> {
        self.handle.set_node_id(self.index, id)?;
        self.id = id.to_string();
        Ok(())
    }

    /// Retrieves a property value for this node.
    pub fn get_value(&self, property: NodeProperty) -> Result<f64> {
        self.handle.get_node_value(self.index, property)
    }

    /// Sets a property value for this node.
    pub fn set_value(&self, property: NodeProperty, value: f64) -> Result<()> {
        self.handle
            .set_node_value(self.index, property, value)
    }

    /// Converts this node into a typed variant.
    pub fn kind(self) -> NodeKind<'a> {
        match self.node_type {
            NodeType::Junction => NodeKind::Junction(Junction { node: self }),
            NodeType::Reservoir => NodeKind::Reservoir(Reservoir { node: self }),
            NodeType::Tank => NodeKind::Tank(Tank { node: self }),
        }
    }
}

/// Typed representation of different kinds of nodes.
pub enum NodeKind<'a> {
    Junction(Junction<'a>),
    Reservoir(Reservoir<'a>),
    Tank(Tank<'a>),
}

/// Junction node wrapper.
pub struct Junction<'a> {
    pub node: Node<'a>,
}

impl<'a> Junction<'a> {
    pub fn base_demand(&self) -> Result<f64> {
        self.node.get_value(NodeProperty::BaseDemand)
    }

    pub fn set_base_demand(&self, value: f64) -> Result<()> {
        self.node.set_value(NodeProperty::BaseDemand, value)
    }
}

/// Reservoir node wrapper.
pub struct Reservoir<'a> {
    pub node: Node<'a>,
}

impl<'a> Reservoir<'a> {
    pub fn elevation(&self) -> Result<f64> {
        self.node.get_value(NodeProperty::Elevation)
    }

    pub fn set_elevation(&self, value: f64) -> Result<()> {
        self.node.set_value(NodeProperty::Elevation, value)
    }
}

/// Tank node wrapper.
pub struct Tank<'a> {
    pub node: Node<'a>,
}

impl<'a> Tank<'a> {
    pub fn tank_level(&self) -> Result<f64> {
        self.node.get_value(NodeProperty::TankLevel)
    }

    pub fn set_tank_level(&self, value: f64) -> Result<()> {
        self.node.set_value(NodeProperty::TankLevel, value)
    }
}

impl<'a> TryFrom<Node<'a>> for Junction<'a> {
    type Error = Node<'a>;
    fn try_from(node: Node<'a>) -> std::result::Result<Self, Self::Error> {
        if node.node_type == NodeType::Junction {
            Ok(Junction { node })
        } else {
            Err(node)
        }
    }
}

impl<'a> TryFrom<Node<'a>> for Reservoir<'a> {
    type Error = Node<'a>;
    fn try_from(node: Node<'a>) -> std::result::Result<Self, Self::Error> {
        if node.node_type == NodeType::Reservoir {
            Ok(Reservoir { node })
        } else {
            Err(node)
        }
    }
}

impl<'a> TryFrom<Node<'a>> for Tank<'a> {
    type Error = Node<'a>;
    fn try_from(node: Node<'a>) -> std::result::Result<Self, Self::Error> {
        if node.node_type == NodeType::Tank {
            Ok(Tank { node })
        } else {
            Err(node)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::impls::test_utils::fixtures::*;

    #[rstest]
    fn node_type_from_index(ph_close: EPANET) {
        let node = Node::new(&ph_close, "TMP", NodeType::Junction).unwrap();
        let idx = node.get_index();
        assert_eq!(node.get_id(), "TMP");
        assert_eq!(node.get_type(), NodeType::Junction);

        let fetched = Node::from_index(&ph_close, idx).unwrap();
        assert_eq!(fetched.get_id(), "TMP");
        assert_eq!(fetched.get_type(), NodeType::Junction);
    }

    #[rstest]
    fn node_junction_from_variant(ph_close: EPANET) {
        let node = Node::new(&ph_close, "TMP", NodeType::Junction).unwrap();
        match node.kind() {
            NodeKind::Junction(j) => {
                let demand = j.base_demand().unwrap();
                assert!(demand >= 0.0);
            }
            _ => panic!("expected junction"),
        }
    }
}
