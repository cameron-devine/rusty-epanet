//! Node-related API methods for EPANET.
//!
//! This module contains methods for adding, deleting, and querying nodes.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::node::{NodeProperty, NodeType};
use crate::types::MAX_MSG_SIZE;
use crate::types::{ActionCodeType, CountType::NodeCount};
use crate::EPANET;
use enum_primitive::FromPrimitive;
use std::ffi::{c_char, c_int, CStr, CString};
use std::mem::MaybeUninit;

/// ## Node APIs
impl EPANET {
    /// Adds a new node to the EPANET model.
    ///
    /// This function creates and adds a new node to the EPANET model with the specified ID
    /// and type. After the node is added, it returns the index of the newly created node in
    /// the model.
    ///
    /// # Parameters
    /// - `id`: The unique identifier for the new node. This should be a valid string and
    ///   unique within the model.
    /// - `node_type`: The type of the node, represented by the [`NodeType`] enum. The node
    ///   type determines the functionality and behavior of the node (e.g., junction, reservoir).
    ///
    /// # Returns
    /// A [`Result<i32>`] which:
    /// - `Ok(i32)` contains the 1-based index of the newly created node in the model.
    /// - `Err(EPANETError)` contains an error if the node addition fails, wrapping the error
    ///   code and additional context about the operation.
    ///
    /// # Implementation Details
    /// - Converts the `id` string into a `CString` to ensure compatibility with the C API.
    /// - Calls the EPANET C API function EN_addnode to add the node and retrieve its index.
    /// - Returns the index of the newly added node on success.
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. While the caller
    /// does not need to manage the safety of the FFI call, it assumes:
    /// - The EPANET model is correctly initialized.
    /// - The `id` string refers to a valid, unique node ID.
    /// - The `node_type` is a valid enumeration value defined in [`NodeType`].
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to add the node. Common
    ///   reasons include:
    ///   - The `id` already exists in the model.
    ///   - The `node_type` is invalid or not applicable.
    /// - Includes additional context in the error message, specifying the node ID and type
    ///   for debugging.
    ///
    /// # See Also
    /// - EN_addnode (EPANET C API)
    /// - [`NodeType`] for possible node types.
    pub fn add_node(&self, id: &str, node_type: NodeType) -> Result<i32> {
        let _id = CString::new(id).unwrap();
        let mut out_index = MaybeUninit::uninit();
        let code = unsafe {
            ffi::EN_addnode(
                self.ph,
                _id.as_ptr(),
                node_type as i32,
                out_index.as_mut_ptr(),
            )
        };
        check_error_with_context(
            code,
            format!("Failed to add node of type {:?} with id {}", node_type, id),
        )?;
        Ok(unsafe { out_index.assume_init() })
    }

    /// Deletes a node from the EPANET model.
    ///
    /// This function removes a node from the EPANET model, identified by its index, and performs
    /// an action specified by the provided [`ActionCodeType`]. The action determines the type
    /// of adjustment made to the surrounding network to maintain consistency after deletion.
    ///
    /// # Parameters
    /// - `id`: The 1-based index of the node to be deleted in the EPANET model.
    /// - `action_code`: The [`ActionCodeType`] specifying the adjustment to be performed on
    ///   the network when the node is deleted (e.g., deleting connecting links or preserving them).
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` indicates the node was successfully deleted.
    /// - `Err(EPANETError)` contains an error if the deletion fails, wrapping the error
    ///   code and additional context about the operation.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function EN_deletenode to delete the node and apply
    ///   the specified action.
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. While the caller
    /// does not need to manage the safety of the FFI call, it assumes:
    /// - The EPANET model is correctly initialized.
    /// - The `id` refers to a valid node index in the model.
    /// - The `action_code` is a valid enumeration value defined in [`ActionCodeType`].
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to delete the node. Common
    ///   reasons include:
    ///   - The `id` does not correspond to a valid node.
    ///   - The `action_code` is invalid or not applicable for the node.
    /// - Includes additional context in the error message, specifying the node ID and action
    ///   code for debugging.
    ///
    /// # See Also
    /// - `EN_deletenode` (EPANET C API)
    /// - [`ActionCodeType`] for possible adjustment actions when deleting a node.
    pub fn delete_node(&self, id: i32, action_code: ActionCodeType) -> Result<()> {
        let code = unsafe { ffi::EN_deletenode(self.ph, id, action_code as i32) };
        check_error_with_context(
            code,
            format!(
                "Failed to delete node with id {} with action code {:?}",
                id, action_code
            ),
        )
    }

    /// Retrieves the index of a node in the EPANET model given its ID.
    ///
    /// This function fetches the 1-based index of a node identified by its ID in the EPANET model.
    /// Node IDs are strings that uniquely identify nodes within the model, while their indices
    /// correspond to their internal position in the EPANET data structure.
    ///
    /// # Parameters
    /// - `id`: The unique identifier of the node whose index is to be retrieved.
    ///
    /// # Returns
    /// A [`Result<i32>`] which:
    /// - `Ok(i32)` contains the 1-based index of the node associated with the given ID.
    /// - `Err(EPANETError)` contains an error if the index retrieval fails, wrapping the error
    ///   code and additional context about the operation.
    ///
    /// # Implementation Details
    /// - Converts the `id` string into a `CString` to ensure compatibility with the C API.
    /// - Uses the EPANET C API function EN_getnodeindex to retrieve the node index.
    /// - Returns the retrieved index on success.
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. While the caller
    /// does not need to manage the safety of the FFI call, it assumes:
    /// - The EPANET model is correctly initialized.
    /// - The `id` string refers to a valid node ID in the model.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to retrieve the node index. Common
    ///   reasons include:
    ///   - The `id` does not correspond to a valid node ID.
    /// - Includes additional context in the error message, specifying the node ID for debugging.
    ///
    /// # See Also
    /// - EN_getnodeindex (EPANET C API)
    pub fn get_node_index(&self, id: &str) -> Result<i32> {
        let _id = CString::new(id).unwrap();
        let mut out_index = MaybeUninit::uninit();
        let code = unsafe { ffi::EN_getnodeindex(self.ph, _id.as_ptr(), out_index.as_mut_ptr()) };
        check_error_with_context(code, format!("Failed to get index for node with id {}", id))?;
        Ok(unsafe { out_index.assume_init() })
    }

    /// Retrieves the ID of a specific node in the EPANET model.
    ///
    /// This function fetches the identifier (ID) of a node in the EPANET model, given the node's
    /// index. Node IDs are strings that uniquely identify nodes within the model.
    ///
    /// # Parameters
    /// - `index`: The 1-based index of the node whose ID is to be retrieved. This corresponds
    ///   to the node's position in the EPANET model.
    ///
    /// # Returns
    /// A [`Result<String>`] which:
    /// - `Ok(String)` contains the ID of the specified node as a string.
    /// - `Err(EPANETError)` contains an error if the ID retrieval fails, wrapping the error
    ///   code and additional context about the operation.
    ///
    /// # Implementation Details
    /// - Allocates a buffer (`Vec<c_char>`) large enough to hold the node ID based on
    ///   the EPANET-defined size limit [`MAX_MSG_SIZE`].
    /// - Calls the EPANET C API function EN_getnodeid to populate the buffer with
    ///   the node ID.
    /// - Converts the resulting C string into a Rust `String` for ergonomic usage.
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. While the caller
    /// does not need to manage the safety of the FFI call, it assumes:
    /// - The EPANET model is correctly initialized.
    /// - The `index` refers to a valid node in the model.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to retrieve the node ID. Common
    ///   reasons include:
    ///   - The `index` does not correspond to a valid node.
    ///   - The buffer size is insufficient (unlikely given the EPANET size limits).
    /// - Includes additional context in the error message, specifying the node index for debugging.
    ///
    /// # See Also
    /// - EN_getnodeid (EPANET C API)
    /// - [`MAX_MSG_SIZE`] for the size limit used for node IDs.
    pub fn get_node_id(&self, index: i32) -> Result<String> {
        let mut out_id: Vec<c_char> = vec![0; MAX_MSG_SIZE as usize + 1usize];
        let code = unsafe { ffi::EN_getnodeid(self.ph, index, out_id.as_mut_ptr()) };
        check_error_with_context(
            code,
            format!("Failed to get node id for node at index {}", index),
        )?;
        Ok(unsafe { CStr::from_ptr(out_id.as_ptr()) }
            .to_string_lossy()
            .to_string())
    }

    /// Changes the ID of a specific node in the EPANET model.
    ///
    /// This function updates the identifier (ID) of a node in the EPANET model, allowing
    /// the user to rename a node by its index. The new ID must be provided as a string
    /// that conforms to EPANET's naming conventions.
    ///
    /// # Parameters
    /// - `index`: The 1-based index of the node to rename in the EPANET model.
    /// - `node_id`: The new ID to assign to the node. This must be a valid string and
    ///   unique within the model.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the node ID is successfully updated.
    /// - `Err(EPANETError)` if an error occurs, wrapping the error code and additional context
    ///   about the operation.
    ///
    /// # Implementation Details
    /// - Converts the `node_id` string into a `CString` to ensure compatibility with the C API.
    /// - Uses the EPANET C API function EN_setnodeid to update the node's ID.
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. While the caller
    /// does not need to manage the safety of the FFI call, it assumes:
    /// - The EPANET model is correctly initialized.
    /// - The `index` refers to a valid node in the model.
    /// - The `node_id` string is valid and adheres to EPANET's naming rules.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to update the node ID. Common
    ///   reasons include:
    ///   - The `index` does not correspond to a valid node.
    ///   - The `node_id` is invalid or conflicts with an existing ID.
    /// - Includes additional context in the error message, specifying the node ID and index
    ///   for debugging.
    ///
    /// # See Also
    /// - EN_setnodeid (EPANET C API)
    pub fn set_node_id(&self, index: i32, node_id: &str) -> Result<()> {
        let _id = CString::new(node_id).unwrap();
        let code = unsafe { ffi::EN_setnodeid(self.ph, index, _id.as_ptr()) };
        check_error_with_context(
            code,
            format!(
                "Failed to set the id of {} for node at index {}",
                node_id, index
            ),
        )
    }

    /// Retrieves the type of a specific node in the EPANET model.
    ///
    /// This function calls the EPANET library to fetch the type of a node identified by its
    /// index. The node type is returned as an enumeration of [`NodeType`], which includes
    /// options such as junctions, reservoirs, and tanks.
    ///
    /// # Parameters
    /// - `index`: The 1-based index of the node in the EPANET model for which the type is to be retrieved.
    ///
    /// # Returns
    /// A [`Result<NodeType>`] which:
    /// - `Ok(NodeType)` contains the node type as an enumeration value.
    /// - `Err(EPANETError)` contains an error if the retrieval fails, wrapping the error code
    ///   and additional context about the operation.
    ///
    /// # Implementation Details
    /// - The function uses the EPANET C API function EN_getnodetype to retrieve
    ///   the type of the node.
    /// - It converts the raw integer type returned by the FFI call into a [`NodeType`]
    ///   enumeration using the `from_i32` method. The function assumes that the
    ///   conversion is valid for all values returned by the EPANET library.
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. While the caller
    /// does not need to handle the safety of the FFI call, it assumes:
    /// - The EPANET model is correctly initialized.
    /// - The `index` refers to a valid node in the model.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to retrieve the node type.
    /// - Includes additional context in the error message, specifying the node index for debugging.
    ///
    /// # See Also
    /// - EN_getnodetype (EPANET C API)
    /// - [`NodeType`] for the list of possible node types returned by this function.
    pub fn get_node_type(&self, index: i32) -> Result<NodeType> {
        let mut node_type: MaybeUninit<c_int> = MaybeUninit::uninit();
        let code = unsafe { ffi::EN_getnodetype(self.ph, index, node_type.as_mut_ptr()) };
        check_error_with_context(
            code,
            format!("Failed to get node type for node at index {}", index),
        )?;
        let init_node_type = unsafe { node_type.assume_init() };
        Ok(NodeType::from_i32(init_node_type).unwrap())
    }

    /// Retrieves the values of a specific property for all nodes in the EPANET model.
    ///
    /// This function fetches the values of a specified property for all nodes in the EPANET model
    /// using the EPANET library. The property to retrieve is specified using the [`NodeProperty`]
    /// enumeration, and the values are returned as a `Vec<f64>`.
    ///
    /// # Parameters
    /// - `node_property`: The [`NodeProperty`] enumeration value specifying the property
    ///   to retrieve for all nodes. For example, this could represent base demand,
    ///   elevation, or pressure.
    ///
    /// # Returns
    /// A [`Result<Vec<f64>>`] which:
    /// - `Ok(Vec<f64>)` contains a vector of property values for all nodes, with each value
    ///   corresponding to a node in the EPANET model.
    /// - `Err(EPANETError)` if an error occurs while retrieving the values, wrapping the
    ///   error code and additional context about the operation.
    ///
    /// # Implementation Details
    /// - This function first determines the total number of nodes in the EPANET model by
    ///   calling [`get_count`] with [`NodeCount`].
    /// - It initializes a vector with the appropriate capacity to store the values for all
    ///   nodes.
    /// - Uses the `ffi::EN_getnodevalues` function to populate the vector with property
    ///   values for all nodes.
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. While the caller
    /// does not need to worry about the safety of the FFI call itself, it assumes:
    /// - The EPANET model is correctly initialized.
    /// - The `node_property` is valid for the EPANET model.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if:
    ///   - The number of nodes cannot be retrieved.
    ///   - The EPANET library fails to retrieve the property values.
    /// - Includes additional context in the error message, such as the specific property
    ///   being retrieved.
    ///
    /// # See Also
    /// - EN_getnodevalues (EPANET C API)
    /// - [`NodeProperty`] for the list of available properties that can be retrieved.
    /// - [`get_count`] for determining the total number of nodes.
    pub fn get_node_values(&self, node_property: NodeProperty) -> Result<Vec<f64>> {
        let node_count = self.get_count(NodeCount)?;
        let mut result: Vec<f64> = vec![0.0; node_count as usize];
        check_error_with_context(
            unsafe { ffi::EN_getnodevalues(self.ph, node_property as i32, result.as_mut_ptr()) },
            format!("Failed to get {:?} for all nodes", node_property),
        )?;
        Ok(result)
    }

    /// Retrieves the value of a specific property for a node in the EPANET model.
    ///
    /// This function calls the EPANET library to fetch the value of a specified property
    /// for a node identified by its index. The property to retrieve is specified using
    /// the [`NodeProperty`] enumeration.
    ///
    /// # Parameters
    /// - `index`: The index of the node for which the property value is to be retrieved.
    ///   The index is 1-based and corresponds to the node in the EPANET model.
    /// - `node_property`: The [`NodeProperty`] enumeration value specifying the property
    ///   to retrieve. For example, this could represent the node's base demand or
    ///   pressure.
    ///
    /// # Returns
    /// A [`Result<f64>`] which:
    /// - `Ok(f64)` contains the retrieved value of the specified node property.
    /// - `Err(EPANETError)` if the EPANET library fails to set the property value, wrapping
    ///   the error code and additional context about the operation
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. The caller
    /// does not need to worry about the safety of the FFI call itself, as this function
    /// ensures proper error handling and type conversion. However, it assumes that the
    /// EPANET model is correctly initialized and that the provided `index` and
    /// `node_property` are valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to retrieve the property value,
    ///   for example, due to an invalid node index or property.
    ///
    /// # See Also
    /// - EN_getnodevalue (EPANET C API)
    pub fn get_node_value(&self, index: i32, node_property: NodeProperty) -> Result<f64> {
        let mut value: MaybeUninit<f64> = MaybeUninit::uninit();
        check_error_with_context(
            unsafe {
                ffi::EN_getnodevalue(self.ph, index, node_property as i32, value.as_mut_ptr())
            },
            format!(
                "Failed to get {:?} for node at index {}",
                node_property, index
            ),
        )?;
        Ok(unsafe { value.assume_init() })
    }

    /// Sets the value of a specific property for a node in the EPANET model.
    ///
    /// This function calls the EPANET library to set the value of a specified property
    /// for a node identified by its index. The property to set is specified using the
    /// [`NodeProperty`] enumeration, and the new value is provided as a `f64`.
    ///
    /// # Parameters
    /// - `index`: The index of the node for which the property value is to be set.
    ///   The index is 1-based and corresponds to the node in the EPANET model. This
    ///   parameter is provided as a `usize` for ergonomic usage in Rust, but is converted
    ///   to `i32` internally for the FFI call.
    /// - `node_property`: The [`NodeProperty`] enumeration value specifying the property
    ///   to set. For example, this could represent the node's base demand or elevation.
    /// - `value`: The new value to assign to the specified node property.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the property value is successfully set.
    /// - `Err(EPANETError)` if the EPANET library fails to set the property value, wrapping
    ///   the error code and additional context about the operation.
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. The caller does
    /// not need to worry about the safety of the FFI call itself, as this function ensures
    /// proper error handling and type conversion. However, it assumes that the EPANET model
    /// is correctly initialized and that the provided `index` and `node_property` are valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to set the property value.
    /// - Additional context is provided in the error message, including the node index
    ///   and property type, to aid in debugging.
    ///
    /// # See Also
    /// - EN_setnodevalue (EPANET C API)
    pub fn set_node_value(
        &self,
        index: usize,
        node_property: NodeProperty,
        value: f64,
    ) -> Result<()> {
        // Convert `usize` to `i32` explicitly for FFI
        let index = index as i32;
        let code = unsafe { ffi::EN_setnodevalue(self.ph, index, node_property as i32, value) };
        check_error_with_context(
            code,
            format!(
                "Failed to set {:?} for node at index {}",
                node_property, index
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use crate::types::node::NodeProperty::*;
    use crate::types::node::NodeType::{Junction, Reservoir, Tank};
    use crate::types::ActionCodeType::Unconditional;
    use rstest::rstest;

    #[rstest]
    fn add_delete_nodes(ph_close: EPANET) {
        let result = ph_close.add_node("N2", Junction);
        assert!(result.is_ok());
        let result = ph_close.add_node("N4", Tank);
        assert!(result.is_ok());
        let result = ph_close.add_node("N3", Reservoir);
        assert!(result.is_ok());
        let result = ph_close.add_node("N1", Junction);
        assert!(result.is_ok());

        let result = ph_close.get_node_index("N1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
        let result = ph_close.get_node_index("N2");
        assert!(result.is_ok());
        let result = ph_close.delete_node(result.unwrap(), Unconditional);
        assert!(result.is_ok());

        let result = ph_close.get_node_index("N4");
        assert!(result.is_ok());
        let result = ph_close.delete_node(result.unwrap(), Unconditional);
        assert!(result.is_ok());
        let result = ph_close.get_node_index("N3");
        assert!(result.is_ok());
        let result = ph_close.delete_node(result.unwrap(), Unconditional);
        assert!(result.is_ok());
    }

    #[rstest]
    fn node_validate_id(ph: EPANET) {
        // Test adding a valid node ID
        let result = ph.add_node("N2", NodeType::Junction);
        assert!(result.is_ok());

        // Test adding a node ID with invalid characters (space)
        let result = ph.add_node("N 3", NodeType::Junction);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), EPANETError::from(252));

        // Test adding a node ID with invalid starting character (quote)
        let result = ph.add_node("\"N3", NodeType::Junction);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), EPANETError::from(252));

        // Test adding a node ID with invalid character (semicolon)
        let result = ph.add_node("N;3", NodeType::Junction);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), EPANETError::from(252));

        // Test renaming a node to an invalid ID
        let index = ph.get_node_index("N2").expect("Node 'N2' should exist");
        let result = ph.set_node_id(index, "N;2");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), EPANETError::from(252));
    }

    #[rstest]
    fn node_junction_properties(ph: EPANET) {
        // Fetch node index for node id "11"
        let index = ph.get_node_index("11").unwrap();
        assert_eq!(ph.get_node_value(index, Elevation).unwrap(), 710.0);
        assert_eq!(ph.get_node_value(index, BaseDemand).unwrap(), 150.0);
        assert_eq!(ph.get_node_value(index, Pattern).unwrap(), 0.0);
        assert_eq!(ph.get_node_value(index, Emitter).unwrap(), 0.0);
        assert_eq!(ph.get_node_value(index, InitQual).unwrap(), 0.5);
    }

    #[rstest]
    fn node_tank_properties(ph: EPANET) {
        use crate::types::node::NodeProperty::{
            Elevation, MaxLevel, MinLevel, MinVolume, TankDiam, TankLevel,
        };

        let index = ph.get_node_index("2").unwrap();

        assert_eq!(ph.get_node_value(index, Elevation).unwrap(), 850.0);
        assert_eq!(ph.get_node_value(index, TankLevel).unwrap(), 120.0);
        assert_eq!(ph.get_node_value(index, MinLevel).unwrap(), 100.0);
        assert_eq!(ph.get_node_value(index, MaxLevel).unwrap(), 150.0);
        assert_eq!(ph.get_node_value(index, TankDiam).unwrap(), 50.5);
        assert!(approx_eq(
            ph.get_node_value(index, MinVolume).unwrap(),
            200296.167,
            1e-3
        ));
    }

    #[rstest]
    fn node_junction_properties_after_step(after_step: EPANET) {
        // Fetch node index for node id "11"
        let index = after_step.get_node_index("11").unwrap();

        assert!(approx_eq(
            after_step.get_node_value(index, Demand).unwrap(),
            179.999,
            1e-3
        ));
        assert!(approx_eq(
            after_step.get_node_value(index, Head).unwrap(),
            991.574,
            1e-3
        ));
        assert!(approx_eq(
            after_step.get_node_value(index, Pressure).unwrap(),
            122.006,
            1e-3
        ));
        assert!(approx_eq(
            after_step.get_node_value(index, Quality).unwrap(),
            0.857,
            1e-3
        ));
    }
}
