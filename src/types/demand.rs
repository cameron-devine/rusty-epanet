use crate::bindings::*;
use crate::{epanet_error::*, EPANET};
use num_derive::FromPrimitive;

/// A struct representing a demand category on a node in an EPANET project.
///
/// Demands are associated with a specific node and represent a category of
/// water demand at that node. Each node can have multiple demand categories,
/// each with its own base demand, time pattern, and name.
///
/// `Demand` instances hold a reference to their parent [`EPANET`] project so
/// that changes can be synchronised back to the engine without having to invoke
/// update functions on the project explicitly. After mutating any of the public
/// fields, call [`Demand::update`] to commit those changes. The demand can
/// also be removed from the model by consuming it with [`Demand::delete`].
///
/// # Example
///
/// ```ignore
/// use epanet::EPANET;
/// use epanet::types::demand::Demand;
///
/// let epanet = EPANET::with_inp_file("network.inp", "", "")?;
///
/// // Create a new demand category on node 1
/// let demand = Demand::new(&epanet, 1, 100.0, "", "Residential")?;
///
/// // Read back a demand from the model
/// let dem = epanet.get_demand(1, "Residential")?;
/// println!("Demand '{}' = {} at node {}", dem.name, dem.base_demand, dem.node_index);
///
/// // Modify and update
/// let mut dem = epanet.get_demand_by_index(1, 1)?;
/// dem.base_demand = 200.0;
/// dem.update()?;
///
/// // Delete
/// dem.delete()?;
/// ```
#[derive(Debug, Clone)]
pub struct Demand<'a> {
    /// Reference to the owning EPANET project.
    pub(crate) project: &'a EPANET,
    /// Index of the node this demand belongs to (1-based).
    pub node_index: i32,
    /// Index of this demand category on the node (1-based).
    pub(crate) demand_index: i32,
    /// Base demand value.
    pub base_demand: f64,
    /// Index of the time pattern applied to this demand (0 = no pattern).
    pub pattern_index: i32,
    /// Name of the demand category.
    pub name: String,
}

impl<'a> Demand<'a> {
    /// Creates a new demand category on a node.
    ///
    /// # Parameters
    ///
    /// - `project`: Reference to the EPANET project
    /// - `node_index`: 1-based index of the node to add the demand to
    /// - `base_demand`: Base demand value
    /// - `demand_pattern`: ID of the time pattern to apply (empty string for no pattern)
    /// - `name`: Name for the demand category
    ///
    /// # Returns
    ///
    /// A [`Demand`] struct with the cached fields populated.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the demand cannot be created (e.g., invalid node
    /// index or non-existent pattern ID).
    pub fn new(
        project: &'a EPANET,
        node_index: i32,
        base_demand: f64,
        demand_pattern: &str,
        name: &str,
    ) -> Result<Self> {
        project.create_demand(node_index, base_demand, demand_pattern, name)
    }

    /// Returns the 1-based demand category index on the node.
    pub fn demand_index(&self) -> i32 {
        self.demand_index
    }

    /// Synchronises any local changes back to the EPANET engine.
    ///
    /// This pushes the current [`base_demand`](Self::base_demand),
    /// [`pattern_index`](Self::pattern_index), and [`name`](Self::name) to the
    /// C engine. Call this after mutating any public fields.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the update fails.
    pub fn update(&self) -> Result<()> {
        self.project
            .set_base_demand(self.node_index, self.demand_index, self.base_demand)?;
        self.project
            .set_demand_pattern(self.node_index, self.demand_index, self.pattern_index)?;
        self.project
            .set_demand_name(self.node_index, self.demand_index, &self.name)
    }

    /// Deletes this demand category from its node.
    ///
    /// This method consumes the demand, preventing further use after deletion.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the demand cannot be deleted.
    pub fn delete(self) -> Result<()> {
        self.project
            .delete_demand(self.node_index, self.demand_index)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum DemandModel {
    /// Demand driven analysis
    Dda = EN_DemandModel_EN_DDA,
    /// Pressure driven analysis
    Pda = EN_DemandModel_EN_PDA,
}

pub struct DemandModelInfo {
    pub demand_type: DemandModel,
    pub pressure_min: f64,
    pub pressure_required: f64,
    pub pressure_exponent: f64,
}
