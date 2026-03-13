use crate::{epanet_error::*, EPANET};

/// A struct representing a time pattern in an EPANET project.
///
/// Time patterns are collections of multipliers applied to base demands,
/// reservoir heads, or water quality sources over time. Each multiplier
/// corresponds to a time period defined by the pattern time step.
///
/// `Pattern` instances hold a reference to their parent [`EPANET`] project so
/// that changes can be synchronised back to the engine without having to invoke
/// update functions on the project explicitly. After mutating any of the public
/// fields, call [`Pattern::update`] to commit those changes. The pattern can
/// also be removed from the model by consuming it with [`Pattern::delete`].
///
/// # Example
///
/// ```ignore
/// use epanet::EPANET;
/// use epanet::types::pattern::Pattern;
///
/// let epanet = EPANET::with_inp_file("network.inp", "", "")?;
///
/// // Create a new pattern with multipliers
/// let pattern = Pattern::new(&epanet, "DemandPat", &[1.0, 1.2, 0.8, 0.6])?;
///
/// // Read back a pattern from the model
/// let pat = epanet.get_pattern("DemandPat")?;
/// println!("Pattern {} has {} multipliers", pat.id, pat.multipliers.len());
///
/// // Modify and update
/// let mut pat = epanet.get_pattern("DemandPat")?;
/// pat.multipliers = vec![1.0, 1.5, 0.5, 0.3];
/// pat.update()?;
///
/// // Delete
/// pat.delete()?;
/// ```
#[derive(Debug, Clone)]
pub struct Pattern<'a> {
    /// Reference to the owning EPANET project.
    pub(crate) project: &'a EPANET,
    /// EPANET project index of the pattern (1-based).
    pub(crate) index: i32,
    /// Pattern ID.
    pub id: String,
    /// Pattern multipliers, one per time period.
    pub multipliers: Vec<f64>,
}

impl<'a> Pattern<'a> {
    /// Creates a new time pattern in the EPANET model.
    ///
    /// # Parameters
    ///
    /// - `project`: Reference to the EPANET project
    /// - `id`: Unique identifier for the pattern
    /// - `multipliers`: Array of multipliers, one per time period
    ///
    /// # Returns
    ///
    /// A [`Pattern`] struct with the cached fields populated.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the pattern cannot be created (e.g., duplicate ID).
    pub fn new(
        project: &'a EPANET,
        id: &str,
        multipliers: &[f64],
    ) -> Result<Self> {
        project.create_pattern(id, multipliers)
    }

    /// Returns the EPANET project index of the pattern (1-based).
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Synchronises any local changes back to the EPANET engine.
    ///
    /// This pushes the current [`id`](Self::id) and [`multipliers`](Self::multipliers)
    /// to the C engine. Call this after mutating any public fields.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the update fails.
    pub fn update(&self) -> Result<()> {
        let current_id = self.project.get_pattern_id(self.index)?;
        if current_id != self.id {
            self.project.set_pattern_id(self.index, &self.id)?;
        }
        self.project.set_pattern(self.index, &self.multipliers)
    }

    /// Deletes this pattern from the EPANET model.
    ///
    /// This method consumes the pattern, preventing further use after deletion.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the pattern cannot be deleted.
    pub fn delete(self) -> Result<()> {
        self.project.delete_pattern(self.index)
    }

    /// Returns the average of the pattern multipliers as computed by the C engine.
    ///
    /// This queries the C API directly rather than computing from the cached
    /// multipliers, so it reflects the current engine state.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if the query fails.
    pub fn average(&self) -> Result<f64> {
        self.project.get_average_pattern_value(self.index)
    }
}
