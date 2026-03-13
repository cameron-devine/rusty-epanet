//! Link-related API methods for EPANET.
//!
//! This module contains methods for adding, deleting, and querying links.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::link::*;
use crate::types::MAX_ID_SIZE;
use crate::types::{ActionCodeType, CountType::LinkCount};
use crate::EPANET;
use num_traits::FromPrimitive;
use std::ffi::{c_char, CString};

/// ## Link APIs
impl EPANET {
    /// Adds a new link to the EPANET model.
    ///
    /// Returns the 1-based index of the newly created link.
    pub fn add_link(
        &self,
        id: &str,
        link_type: LinkType,
        from_node: &str,
        to_node: &str,
    ) -> Result<i32> {
        let c_id = CString::new(id)?;
        let c_from = CString::new(from_node)?;
        let c_to = CString::new(to_node)?;
        let mut out_index = 0;

        check_error_with_context(
            unsafe {
                ffi::EN_addlink(
                    self.ph,
                    c_id.as_ptr(),
                    link_type as i32,
                    c_from.as_ptr(),
                    c_to.as_ptr(),
                    &mut out_index,
                )
            },
            format!(
                "Failed to add link '{}' of type {:?} from '{}' to '{}'",
                id, link_type, from_node, to_node
            ),
        )?;

        Ok(out_index)
    }

    /// Retrieves a Link by its ID.
    pub fn get_link(&self, id: &str) -> Result<Link<'_>> {
        let index = self.get_link_index(id)?;
        self.get_link_by_index(index)
    }

    /// Retrieves a Link by its index.
    pub fn get_link_by_index(&self, index: i32) -> Result<Link<'_>> {
        let id = self.get_link_id(index)?;
        let link_type = self.get_link_type(index)?;
        let (from_node, to_node) = self.get_link_nodes(index)?;
        let status = LinkStatusType::from_i32(
            self.get_link_value(index, LinkProperty::Status)? as i32
        ).unwrap_or(LinkStatusType::Open);

        let kind = match link_type {
            LinkType::Pipe => LinkKind::Pipe(PipeData {
                length: self.get_link_value(index, LinkProperty::Length)?,
                diameter: self.get_link_value(index, LinkProperty::Diameter)?,
                roughness: self.get_link_value(index, LinkProperty::Roughness)?,
                minor_loss: self.get_link_value(index, LinkProperty::MinorLoss)?,
            }),
            LinkType::CvPipe => LinkKind::CvPipe(PipeData {
                length: self.get_link_value(index, LinkProperty::Length)?,
                diameter: self.get_link_value(index, LinkProperty::Diameter)?,
                roughness: self.get_link_value(index, LinkProperty::Roughness)?,
                minor_loss: self.get_link_value(index, LinkProperty::MinorLoss)?,
            }),
            LinkType::Pump => {
                let pump_type = self.get_pump_type(index)?;
                let head_curve_idx = self.get_head_curve_index(index).ok();
                let head_curve_index = if head_curve_idx == Some(0) {
                    None
                } else {
                    head_curve_idx
                };

                let efficiency_curve_idx =
                    self.get_link_value(index, LinkProperty::PumpECurve)? as i32;
                let efficiency_curve_index = if efficiency_curve_idx == 0 {
                    None
                } else {
                    Some(efficiency_curve_idx)
                };

                let energy_pattern_idx =
                    self.get_link_value(index, LinkProperty::PumpEPat)? as i32;
                let energy_pattern_index = if energy_pattern_idx == 0 {
                    None
                } else {
                    Some(energy_pattern_idx)
                };

                LinkKind::Pump(PumpData {
                    pump_type,
                    power: self.get_link_value(index, LinkProperty::PumpPower)?,
                    speed: self.get_link_value(index, LinkProperty::InitSetting)?,
                    head_curve_index,
                    efficiency_curve_index,
                    energy_pattern_index,
                    energy_cost: self.get_link_value(index, LinkProperty::PumpECost)?,
                })
            }
            LinkType::Prv
            | LinkType::Psv
            | LinkType::Pbv
            | LinkType::Fcv
            | LinkType::Tcv
            | LinkType::Gpv
            | LinkType::Pcv => {
                let curve_idx = if link_type == LinkType::Gpv {
                    let idx = self.get_link_value(index, LinkProperty::GPVCurve)? as i32;
                    if idx == 0 { None } else { Some(idx) }
                } else if link_type == LinkType::Pcv {
                    let idx = self.get_link_value(index, LinkProperty::PCVCurve)? as i32;
                    if idx == 0 { None } else { Some(idx) }
                } else {
                    None
                };

                LinkKind::Valve(ValveData {
                    diameter: self.get_link_value(index, LinkProperty::Diameter)?,
                    setting: self.get_link_value(index, LinkProperty::InitSetting)?,
                    curve_index: curve_idx,
                })
            }
        };

        Ok(Link {
            project: self,
            index,
            id,
            from_node,
            to_node,
            status,
            kind,
        })
    }

    pub fn delete_link(&self, index: i32, action_code_type: ActionCodeType) -> Result<()> {
        check_error(unsafe { ffi::EN_deletelink(self.ph, index, action_code_type as i32) })
    }

    pub fn get_link_index(&self, id: &str) -> Result<i32> {
        let c_id = CString::new(id).unwrap();
        let mut out_index = 0;
        check_error(unsafe { ffi::EN_getlinkindex(self.ph, c_id.as_ptr(), &mut out_index) })?;
        Ok(out_index)
    }

    pub fn get_link_id(&self, index: i32) -> Result<String> {
        let mut out_id: Vec<c_char> = vec![0; MAX_ID_SIZE as usize + 1usize];
        check_error(unsafe { ffi::EN_getlinkid(self.ph, index, out_id.as_mut_ptr()) })?;
        let id = unsafe { std::ffi::CStr::from_ptr(out_id.as_ptr()) }
            .to_string_lossy()
            .trim_end()
            .to_string();
        Ok(id)
    }

    pub fn set_link_id(&self, index: i32, id: &str) -> Result<()> {
        let c_id = CString::new(id).unwrap();
        check_error(unsafe { ffi::EN_setlinkid(self.ph, index, c_id.as_ptr()) })
    }

    pub fn get_link_type(&self, index: i32) -> Result<LinkType> {
        let mut out_type = 0;
        check_error(unsafe { ffi::EN_getlinktype(self.ph, index, &mut out_type) })?;
        Ok(LinkType::from_i32(out_type).unwrap())
    }

    pub fn set_link_type(
        &self,
        index: i32,
        link_type: LinkType,
        action_code: ActionCodeType,
    ) -> Result<i32> {
        let mut in_out_index = index;
        let result = unsafe {
            ffi::EN_setlinktype(
                self.ph,
                &mut in_out_index,
                link_type as i32,
                action_code as i32,
            )
        };
        check_error(result)?;
        Ok(in_out_index)
    }

    pub fn get_link_nodes(&self, index: i32) -> Result<(i32, i32)> {
        let (mut out_node1, mut out_node2) = (0, 0);
        check_error(unsafe {
            ffi::EN_getlinknodes(self.ph, index, &mut out_node1, &mut out_node2)
        })?;
        Ok((out_node1, out_node2))
    }

    pub fn set_link_nodes(&self, index: i32, node1: i32, node2: i32) -> Result<()> {
        check_error(unsafe { ffi::EN_setlinknodes(self.ph, index, node1, node2) })
    }

    pub fn get_link_value(&self, index: i32, property: LinkProperty) -> Result<f64> {
        let mut out_value = 0.0;
        check_error(unsafe {
            ffi::EN_getlinkvalue(self.ph, index, property as i32, &mut out_value)
        })?;
        Ok(out_value)
    }

    pub fn get_link_values(&self, property: LinkProperty) -> Result<Vec<f64>> {
        let link_count = self.get_count(LinkCount)?;
        let mut values: Vec<f64> = vec![0.0; link_count as usize];
        check_error(unsafe { ffi::EN_getlinkvalues(self.ph, property as i32, values.as_mut_ptr()) })?;
        Ok(values)
    }

    pub fn set_link_value(&self, index: i32, property: LinkProperty, value: f64) -> Result<()> {
        check_error(unsafe { ffi::EN_setlinkvalue(self.ph, index, property as i32, value) })
    }

    pub fn set_pipe_data(
        &self,
        index: i32,
        length: f64,
        diameter: f64,
        roughness: f64,
        minor_loss: f64,
    ) -> Result<()> {
        check_error(unsafe { ffi::EN_setpipedata(self.ph, index, length, diameter, roughness, minor_loss) })
    }

    pub fn get_pump_type(&self, index: i32) -> Result<PumpType> {
        let mut out_type = 0;
        check_error(unsafe { ffi::EN_getpumptype(self.ph, index, &mut out_type) })?;
        Ok(PumpType::from_i32(out_type).unwrap())
    }

    pub fn get_head_curve_index(&self, link_index: i32) -> Result<i32> {
        let mut out_index = 0;
        check_error(unsafe { ffi::EN_getheadcurveindex(self.ph, link_index, &mut out_index) })?;
        Ok(out_index)
    }

    pub fn set_head_curve_index(&self, link_index: i32, curve_index: i32) -> Result<()> {
        check_error(unsafe { ffi::EN_setheadcurveindex(self.ph, link_index, curve_index) })
    }

    pub fn get_vertex_count(&self, link_index: i32) -> Result<i32> {
        let mut out_count = 0;
        check_error(unsafe { ffi::EN_getvertexcount(self.ph, link_index, &mut out_count) })?;
        Ok(out_count)
    }

    pub fn get_vertex(&self, link_index: i32, vertex_index: i32) -> Result<(f64, f64)> {
        let (mut out_x, mut out_y) = (0.0, 0.0);
        check_error(unsafe {
            ffi::EN_getvertex(self.ph, link_index, vertex_index, &mut out_x, &mut out_y)
        })?;
        Ok((out_x, out_y))
    }

    pub fn set_vertex(&self, link_index: i32, vertex_index: i32, x: f64, y: f64) -> Result<()> {
        check_error(unsafe { ffi::EN_setvertex(self.ph, link_index, vertex_index, x, y) })
    }

    pub fn set_vertices(&self, link_index: i32, vertices: Vec<(f64, f64)>) -> Result<()> {
        let (mut xs, mut ys): (Vec<f64>, Vec<f64>) = vertices.iter().cloned().unzip();
        check_error(unsafe {
            ffi::EN_setvertices(
                self.ph,
                link_index,
                xs.as_mut_ptr(),
                ys.as_mut_ptr(),
                vertices.len() as i32,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::test_utils::fixtures::*;
    use crate::types::link::*;
    use crate::types::node::Node;
    use crate::types::ActionCodeType::Unconditional;
    use crate::types::CountType::LinkCount;
    use crate::EPANET;
    use rstest::rstest;

    #[rstest]
    fn test_add_delete_links(ph_close: EPANET) {
        let _n1 = Node::new_junction(&ph_close, "N1", 100.0, 50.0, "").unwrap();
        let _n2 = Node::new_junction(&ph_close, "N2", 100.0, 50.0, "").unwrap();

        let idx1 = ph_close.add_link("P1", LinkType::Pipe, "N1", "N2").unwrap();
        assert_eq!(idx1, 1);

        let idx2 = ph_close.add_link("PMP1", LinkType::Pump, "N1", "N2").unwrap();
        assert_eq!(idx2, 2);

        assert_eq!(ph_close.get_count(LinkCount).unwrap(), 2);

        ph_close.delete_link(1, Unconditional).unwrap();
        assert_eq!(ph_close.get_count(LinkCount).unwrap(), 1);
    }

    #[rstest]
    fn test_link_index_and_id(ph: EPANET) {
        let index = ph.get_link_index("10").unwrap();
        assert!(index > 0);
        assert_eq!(ph.get_link_id(index).unwrap(), "10");
        assert!(ph.get_link_index("nonexistent").is_err());
    }

    #[rstest]
    fn test_set_link_id(ph: EPANET) {
        let index = ph.get_link_index("10").unwrap();
        ph.set_link_id(index, "RENAMED").unwrap();
        assert_eq!(ph.get_link_id(index).unwrap(), "RENAMED");
        assert_eq!(ph.get_link_index("RENAMED").unwrap(), index);
    }

    #[rstest]
    fn test_link_type_get_set(ph_close: EPANET) {
        let _n1 = Node::new_junction(&ph_close, "N1", 100.0, 50.0, "").unwrap();
        let _n2 = Node::new_junction(&ph_close, "N2", 100.0, 50.0, "").unwrap();

        let index = ph_close.add_link("P1", LinkType::Pipe, "N1", "N2").unwrap();
        assert_eq!(ph_close.get_link_type(index).unwrap(), LinkType::Pipe);

        let new_index = ph_close.set_link_type(index, LinkType::CvPipe, Unconditional).unwrap();
        assert_eq!(ph_close.get_link_type(new_index).unwrap(), LinkType::CvPipe);
    }

    #[rstest]
    fn test_link_nodes_get_set(ph: EPANET) {
        let link_index = ph.get_link_index("10").unwrap();
        let (n1, n2) = ph.get_link_nodes(link_index).unwrap();
        assert_eq!(n1, ph.get_node_index("10").unwrap());
        assert_eq!(n2, ph.get_node_index("11").unwrap());

        // Swap the nodes
        ph.set_link_nodes(link_index, n2, n1).unwrap();
        let (new_n1, new_n2) = ph.get_link_nodes(link_index).unwrap();
        assert_eq!(new_n1, n2);
        assert_eq!(new_n2, n1);
    }

    #[rstest]
    fn test_link_value_get_set(ph: EPANET) {
        let index = ph.get_link_index("10").unwrap();
        assert!(approx_eq(ph.get_link_value(index, LinkProperty::Length).unwrap(), 10530.0, 1.0));
        assert!(approx_eq(ph.get_link_value(index, LinkProperty::Diameter).unwrap(), 18.0, 0.1));
        assert!(approx_eq(ph.get_link_value(index, LinkProperty::Roughness).unwrap(), 100.0, 0.1));

        ph.set_link_value(index, LinkProperty::Roughness, 120.0).unwrap();
        assert!(approx_eq(ph.get_link_value(index, LinkProperty::Roughness).unwrap(), 120.0, 0.1));
    }

    #[rstest]
    fn test_link_values_batch_and_pipe_data(ph: EPANET) {
        let link_count = ph.get_count(LinkCount).unwrap();
        let diameters = ph.get_link_values(LinkProperty::Diameter).unwrap();
        assert_eq!(diameters.len(), link_count as usize);

        let index = ph.get_link_index("10").unwrap();
        ph.set_pipe_data(index, 5000.0, 16.0, 110.0, 0.5).unwrap();
        assert!(approx_eq(ph.get_link_value(index, LinkProperty::Length).unwrap(), 5000.0, 0.1));
        assert!(approx_eq(ph.get_link_value(index, LinkProperty::Diameter).unwrap(), 16.0, 0.1));
        assert!(approx_eq(ph.get_link_value(index, LinkProperty::Roughness).unwrap(), 110.0, 0.1));
        assert!(approx_eq(ph.get_link_value(index, LinkProperty::MinorLoss).unwrap(), 0.5, 0.01));
    }

    #[rstest]
    fn test_pump_type_and_head_curve(ph: EPANET) {
        let index = ph.get_link_index("9").unwrap();
        let pump_type = ph.get_pump_type(index).unwrap();
        assert!(pump_type == PumpType::Custom || pump_type == PumpType::PowerFunc || pump_type == PumpType::ConstHp || pump_type == PumpType::NoCurve);

        let curve_index = ph.get_head_curve_index(index).unwrap();
        assert!(curve_index > 0);

        ph.set_head_curve_index(index, curve_index).unwrap();
        assert_eq!(ph.get_head_curve_index(index).unwrap(), curve_index);
    }

    #[rstest]
    fn test_vertices(ph_close: EPANET) {
        let _n1 = Node::new_junction(&ph_close, "N1", 100.0, 50.0, "").unwrap();
        let _n2 = Node::new_junction(&ph_close, "N2", 100.0, 50.0, "").unwrap();
        let link_index = ph_close.add_link("P1", LinkType::Pipe, "N1", "N2").unwrap();

        assert_eq!(ph_close.get_vertex_count(link_index).unwrap(), 0);

        ph_close.set_vertices(link_index, vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]).unwrap();
        assert_eq!(ph_close.get_vertex_count(link_index).unwrap(), 3);

        let (x, y) = ph_close.get_vertex(link_index, 1).unwrap();
        assert!(approx_eq(x, 1.0, 0.001));
        assert!(approx_eq(y, 2.0, 0.001));

        ph_close.set_vertex(link_index, 2, 10.0, 20.0).unwrap();
        let (x2, y2) = ph_close.get_vertex(link_index, 2).unwrap();
        assert!(approx_eq(x2, 10.0, 0.001));
        assert!(approx_eq(y2, 20.0, 0.001));
    }
}
