//! Collection methods for fetching all objects of a given type.
//!
//! These methods provide convenient access to all nodes, links, patterns,
//! curves, controls, and rules in the EPANET model, as well as filtered
//! convenience methods for specific subtypes (junctions, tanks, pipes, etc.).

use crate::epanet_error::*;
use crate::types::control::Control;
use crate::types::curve::Curve;
use crate::types::link::Link;
use crate::types::node::Node;
use crate::types::pattern::Pattern;
use crate::types::rule::Rule;
use crate::types::CountType;
use crate::EPANET;

/// ## Collection APIs
impl EPANET {
    /// Fetches all nodes in the model.
    ///
    /// Returns a `Vec<Node>` containing every node (junctions, tanks, and
    /// reservoirs) in the current EPANET project, ordered by their 1-based
    /// index.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any node cannot be retrieved.
    pub fn nodes(&self) -> Result<Vec<Node<'_>>> {
        let count = self.get_count(CountType::NodeCount)?;
        let mut nodes = Vec::with_capacity(count as usize);
        for i in 1..=count {
            nodes.push(self.get_node_by_index(i)?);
        }
        Ok(nodes)
    }

    /// Fetches all links in the model.
    ///
    /// Returns a `Vec<Link>` containing every link (pipes, pumps, and valves)
    /// in the current EPANET project, ordered by their 1-based index.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any link cannot be retrieved.
    pub fn links(&self) -> Result<Vec<Link<'_>>> {
        let count = self.get_count(CountType::LinkCount)?;
        let mut links = Vec::with_capacity(count as usize);
        for i in 1..=count {
            links.push(self.get_link_by_index(i)?);
        }
        Ok(links)
    }

    /// Fetches all pipe links in the model (including check-valve pipes).
    ///
    /// Convenience method that returns only links where [`Link::is_pipe()`] is true.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any link cannot be retrieved.
    pub fn pipes(&self) -> Result<Vec<Link<'_>>> {
        Ok(self.links()?.into_iter().filter(|l| l.is_pipe()).collect())
    }

    /// Fetches all pump links in the model.
    ///
    /// Convenience method that returns only links where [`Link::is_pump()`] is true.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any link cannot be retrieved.
    pub fn pumps(&self) -> Result<Vec<Link<'_>>> {
        Ok(self.links()?.into_iter().filter(|l| l.is_pump()).collect())
    }

    /// Fetches all valve links in the model.
    ///
    /// Convenience method that returns only links where [`Link::is_valve()`] is true.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any link cannot be retrieved.
    pub fn valves(&self) -> Result<Vec<Link<'_>>> {
        Ok(self.links()?.into_iter().filter(|l| l.is_valve()).collect())
    }

    /// Fetches all junction nodes in the model.
    ///
    /// Convenience method that returns only nodes where [`Node::is_junction()`] is true.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any node cannot be retrieved.
    pub fn junctions(&self) -> Result<Vec<Node<'_>>> {
        Ok(self
            .nodes()?
            .into_iter()
            .filter(|n| n.is_junction())
            .collect())
    }

    /// Fetches all tank nodes in the model.
    ///
    /// Convenience method that returns only nodes where [`Node::is_tank()`] is true.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any node cannot be retrieved.
    pub fn tanks(&self) -> Result<Vec<Node<'_>>> {
        Ok(self
            .nodes()?
            .into_iter()
            .filter(|n| n.is_tank())
            .collect())
    }

    /// Fetches all time patterns in the model.
    ///
    /// Returns a `Vec<Pattern>` containing every pattern in the current
    /// EPANET project, ordered by their 1-based index.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any pattern cannot be retrieved.
    pub fn patterns(&self) -> Result<Vec<Pattern<'_>>> {
        let count = self.get_count(CountType::PatternCount)?;
        let mut patterns = Vec::with_capacity(count as usize);
        for i in 1..=count {
            patterns.push(self.get_pattern_by_index(i)?);
        }
        Ok(patterns)
    }

    /// Fetches all curves in the model.
    ///
    /// Returns a `Vec<Curve>` containing every curve in the current EPANET
    /// project, ordered by their 1-based index.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any curve cannot be retrieved.
    pub fn curves(&self) -> Result<Vec<Curve<'_>>> {
        let count = self.get_count(CountType::CurveCount)?;
        let mut curves = Vec::with_capacity(count as usize);
        for i in 1..=count {
            curves.push(self.get_curve_by_index(i)?);
        }
        Ok(curves)
    }

    /// Fetches all simple controls in the model.
    ///
    /// Returns a `Vec<Control>` containing every simple control in the current
    /// EPANET project, ordered by their 1-based index.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any control cannot be retrieved.
    pub fn controls(&self) -> Result<Vec<Control<'_>>> {
        let count = self.get_count(CountType::ControlCount)?;
        let mut controls = Vec::with_capacity(count as usize);
        for i in 1..=count {
            controls.push(self.get_control_by_index(i)?);
        }
        Ok(controls)
    }

    /// Fetches all rule-based controls in the model.
    ///
    /// Returns a `Vec<Rule>` containing every rule-based control in the
    /// current EPANET project, ordered by their 1-based index.
    ///
    /// # Errors
    ///
    /// Returns an [`EPANETError`] if any rule cannot be retrieved.
    pub fn rules(&self) -> Result<Vec<Rule<'_>>> {
        let count = self.get_count(CountType::RuleCount)?;
        let mut rules = Vec::with_capacity(count as usize);
        for i in 1..=count {
            rules.push(self.get_rule(i)?);
        }
        Ok(rules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use crate::types::curve::Curve;
    use rstest::rstest;

    #[rstest]
    fn test_nodes(ph: EPANET) {
        let nodes = ph.nodes().unwrap();
        let count = ph.get_count(CountType::NodeCount).unwrap();
        assert_eq!(nodes.len(), count as usize);

        // net1.inp has junctions, a tank, and a reservoir
        assert!(nodes.iter().any(|n| n.is_junction()));
        assert!(nodes.iter().any(|n| n.is_tank()));
        assert!(nodes.iter().any(|n| n.is_reservoir()));
    }

    #[rstest]
    fn test_links(ph: EPANET) {
        let links = ph.links().unwrap();
        let count = ph.get_count(CountType::LinkCount).unwrap();
        assert_eq!(links.len(), count as usize);

        // net1.inp has pipes and a pump
        assert!(links.iter().any(|l| l.is_pipe()));
        assert!(links.iter().any(|l| l.is_pump()));
    }

    #[rstest]
    fn test_pipes(ph: EPANET) {
        let pipes = ph.pipes().unwrap();
        assert!(!pipes.is_empty());
        assert!(pipes.iter().all(|l| l.is_pipe()));
    }

    #[rstest]
    fn test_pumps(ph: EPANET) {
        let pumps = ph.pumps().unwrap();
        assert!(!pumps.is_empty());
        assert!(pumps.iter().all(|l| l.is_pump()));
    }

    #[rstest]
    fn test_valves(ph: EPANET) {
        // net1.inp may not have valves, but the call should succeed
        let valves = ph.valves().unwrap();
        assert!(valves.iter().all(|l| l.is_valve()));
    }

    #[rstest]
    fn test_junctions(ph: EPANET) {
        let junctions = ph.junctions().unwrap();
        assert!(!junctions.is_empty());
        assert!(junctions.iter().all(|n| n.is_junction()));
    }

    #[rstest]
    fn test_tanks(ph: EPANET) {
        let tanks = ph.tanks().unwrap();
        assert!(!tanks.is_empty());
        assert!(tanks.iter().all(|n| n.is_tank()));
    }

    #[rstest]
    fn test_patterns(ph: EPANET) {
        let patterns = ph.patterns().unwrap();
        let count = ph.get_count(CountType::PatternCount).unwrap();
        assert_eq!(patterns.len(), count as usize);
    }

    #[rstest]
    fn test_curves(ph: EPANET) {
        // Add a curve so we have at least one
        let _curve = Curve::new_pump_curve(&ph, "TestC", &[(0.0, 100.0), (500.0, 50.0)]).unwrap();

        let curves = ph.curves().unwrap();
        assert!(!curves.is_empty());
    }

    #[rstest]
    fn test_controls(ph: EPANET) {
        // net1.inp has simple controls
        let controls = ph.controls().unwrap();
        let count = ph.get_count(CountType::ControlCount).unwrap();
        assert_eq!(controls.len(), count as usize);
    }

    #[rstest]
    fn test_rules(ph: EPANET) {
        // net1.inp may not have rules by default
        let rules = ph.rules().unwrap();
        let count = ph.get_count(CountType::RuleCount).unwrap();
        assert_eq!(rules.len(), count as usize);
    }

    #[rstest]
    fn test_filtered_counts_consistent(ph: EPANET) {
        let all_nodes = ph.nodes().unwrap();
        let junctions = ph.junctions().unwrap();
        let tanks = ph.tanks().unwrap();
        let reservoirs: Vec<_> = all_nodes.iter().filter(|n| n.is_reservoir()).collect();

        // Filtered subsets should sum to total
        assert_eq!(
            junctions.len() + tanks.len() + reservoirs.len(),
            all_nodes.len()
        );

        let all_links = ph.links().unwrap();
        let pipes = ph.pipes().unwrap();
        let pumps = ph.pumps().unwrap();
        let valves = ph.valves().unwrap();

        assert_eq!(
            pipes.len() + pumps.len() + valves.len(),
            all_links.len()
        );
    }
}
