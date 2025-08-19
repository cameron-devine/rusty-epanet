//! Rule Based Control-related API methods for EPANET.
//!
//! This module contains methods for getting and adding rule based controls.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::MAX_ID_SIZE;
use crate::types::rule::*;
use crate::EPANET;
use enum_primitive::*;
use std::ffi::c_char;

/// ## Rule baesd Control APIs
impl EPANET {
    pub fn add_rule(&self, rule: &str) -> Result<()> {
        let c_rule = std::ffi::CString::new(rule).unwrap();
        let result =
            unsafe { ffi::EN_addrule(self.ph, c_rule.as_ptr() as *mut std::os::raw::c_char) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn delete_rule(&self, index: i32) -> Result<()> {
        let result = unsafe { ffi::EN_deleterule(self.ph, index) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    pub fn get_rule(&self, index: i32) -> Result<Rule> {
        let rule_id = self.get_rule_id(index)?;

        let mut out_premise_count = 0;
        let mut out_then_action_count = 0;
        let mut out_else_action_count = 0;
        let mut out_priority = 0.0;
        let get_rule_result = unsafe {
            ffi::EN_getrule(
                self.ph,
                index,
                &mut out_premise_count,
                &mut out_then_action_count,
                &mut out_else_action_count,
                &mut out_priority,
            )
        };

        if get_rule_result != 0 {
            return Err(EPANETError::from(get_rule_result));
        }

        let mut premises = Vec::new();
        for i in 1..=out_premise_count {
            let prem_result = self.get_premise(index, i);
            if prem_result.is_ok() {
                premises.push(prem_result?);
            } else {
                return Err(prem_result.err().unwrap());
            }
        }

        let mut then_actions = Vec::new();
        for i in 1..=out_then_action_count {
            let action_result = self.get_then_action(index, i);
            if action_result.is_ok() {
                then_actions.push(action_result?);
            } else {
                return Err(action_result.err().unwrap());
            }
        }

        let mut else_actions = Vec::new();
        for i in 1..=out_else_action_count {
            let action_result = self.get_else_action(index, i);
            if action_result.is_ok() {
                else_actions.push(action_result?);
            } else {
                return Err(action_result.err().unwrap());
            }
        }

        let enabled = self.get_rule_enabled(index)?;

        Ok(Rule {
            rule_id,
            premises,
            then_actions,
            else_actions: if else_actions.len() == 0 {
                None
            } else {
                Some(else_actions)
            },
            priority: None,
            enabled,
        })
    }

    fn get_rule_id(&self, rule_index: i32) -> Result<String> {
        let mut out_rule_id: Vec<c_char> = vec![0; MAX_ID_SIZE as usize + 1usize];
        let result = unsafe { ffi::EN_getruleID(self.ph, rule_index, out_rule_id.as_mut_ptr()) };
        if result == 0 {
            let id = unsafe { std::ffi::CStr::from_ptr(out_rule_id.as_ptr()) }
                .to_string_lossy()
                .trim_end()
                .to_string();
            Ok(id)
        } else {
            Err(EPANETError::from(result))
        }
    }
    fn get_premise(&self, rule_index: i32, premise_index: i32) -> Result<Premise> {
        let mut out_logop = 0;
        let mut out_object = 0;
        let mut out_obj_index = 0;
        let mut out_variable = 0;
        let mut out_relop = 0;
        let mut out_status = 0;
        let mut out_value = 0.0;

        let premise_result = unsafe {
            ffi::EN_getpremise(
                self.ph,
                rule_index,
                premise_index,
                &mut out_logop,
                &mut out_object,
                &mut out_obj_index,
                &mut out_variable,
                &mut out_relop,
                &mut out_status,
                &mut out_value,
            )
        };

        let logical_operator =
            LogicalOperator::from_i32(out_logop).expect("Invalid logical operator");
        let rule_object = RuleObject::from_i32(out_object).expect("Invalid rule object");
        let object_index = out_obj_index;
        let variable = RuleVariable::from_i32(out_variable).expect("Invalid rule variable");
        let rule_operator = RuleOperator::from_i32(out_relop).expect("Invalid rule operator");
        let status: Option<RuleStatus> = RuleStatus::from_i32(out_status).or(None);
        let value = out_value;

        if premise_result == 0 {
            Ok(Premise {
                logical_operator,
                rule_object,
                object_index,
                variable,
                rule_operator,
                status,
                value,
            })
        } else {
            Err(EPANETError::from(premise_result))
        }
    }

    fn get_then_action(&self, rule_index: i32, action_index: i32) -> Result<ActionClause> {
        let mut out_link_index = 0;
        let mut out_status = 0;
        let mut out_setting = 0.0;
        let result = unsafe {
            ffi::EN_getthenaction(
                self.ph,
                rule_index,
                action_index,
                &mut out_link_index,
                &mut out_status,
                &mut out_setting,
            )
        };
        if result == 0 {
            Ok(ActionClause {
                link_index: out_link_index,
                status: RuleStatus::from_i32(out_status).expect("Invalid rule status"),
                setting: out_setting,
            })
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn get_else_action(&self, rule_index: i32, action_index: i32) -> Result<ActionClause> {
        let mut out_link_index = 0;
        let mut out_status = 0;
        let mut out_setting = 0.0;
        let result = unsafe {
            ffi::EN_getelseaction(
                self.ph,
                rule_index,
                action_index,
                &mut out_link_index,
                &mut out_status,
                &mut out_setting,
            )
        };
        if result == 0 {
            Ok(ActionClause {
                link_index: out_link_index,
                status: RuleStatus::from_i32(out_status).expect("Invalid rule status"),
                setting: out_setting,
            })
        } else {
            Err(EPANETError::from(result))
        }
    }

    fn get_rule_enabled(&self, rule_index: i32) -> Result<bool> {
        let mut out_enabled = 0;
        let result = unsafe { ffi::EN_getruleenabled(self.ph, rule_index, &mut out_enabled) };
        if result == 0 {
            Ok(out_enabled != 0)
        } else {
            Err(EPANETError::from(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use crate::types::types::ActionCodeType::{Conditional, Unconditional};
    use crate::types::types::CountType::RuleCount;
    use rstest::rstest;

    const R1: &str = "RULE 1 \n IF NODE 2 LEVEL < 100 \n THEN LINK 9 STATUS = OPEN";
    const R2: &str =
        "RULE 2\nIF SYSTEM TIME = 4\nTHEN LINK 9 STATUS = CLOSED\nAND LINK 31 STATUS = CLOSED";
    const R3: &str = "RULE 3\nIF NODE 23 PRESSURE ABOVE 140\nAND NODE 2 LEVEL > 120\n
    THEN LINK 113 STATUS = CLOSED\nELSE LINK 22 STATUS = CLOSED";

    #[rstest]
    pub fn test_add_get_rule(ph: EPANET) {
        // Add the 3 rules to the project
        let mut add_result = ph.add_rule(R1);
        assert!(add_result.is_ok());
        add_result = ph.add_rule(R2);
        assert!(add_result.is_ok());
        add_result = ph.add_rule(R3);
        assert!(add_result.is_ok());

        // Check that rules were added
        let count_result = ph.get_count(RuleCount);
        assert!(count_result.is_ok());
        assert_eq!(count_result.unwrap(), 3);

        // Check the number of clauses in rule 3
        let get_rule_result = ph.get_rule(3);
        assert!(get_rule_result.is_ok());
        let rule = get_rule_result.unwrap();
        assert_eq!(rule.premises.len(), 2);
        assert_eq!(rule.then_actions.len(), 1);
        assert_eq!(rule.else_actions.unwrap().len(), 1);

        // Try to delete link 113 conditionally which will fail
        // because it's in rule 3
        let link_index_result = ph.get_link_index("113");
        assert!(link_index_result.is_ok());
        let link_index = link_index_result.unwrap();
        assert_eq!(link_index, 10);
        let delete_result = ph.delete_link(link_index, Conditional);
        assert!(delete_result.is_err());
        assert_eq!(delete_result.err().unwrap().code, 261);

        // Delete node 23 unconditionally which will result in the
        // deletion of rule 3 as well as links 22 and 113
        let node23 = ph.get_node_index("23").unwrap();
        let pump9_before = ph.get_link_index("9").unwrap();
        let delete_result = ph.delete_node(node23, Unconditional);
        assert!(delete_result.is_ok());

        // Check that there are now only 2 rules
        let rule_count = ph.get_count(RuleCount).unwrap();
        assert_eq!(rule_count, 2);

        // Check that link 22 no longer exists
        let link22_error = ph.get_link_index("22");
        assert!(link22_error.is_err());
        assert_eq!(link22_error.err().unwrap().code, 204);

        // Check that the index of pump9 has been reduced by 2
        let pump9_after = ph.get_link_index("9").unwrap();
        assert_eq!(pump9_before - pump9_after, 2);
    }
}
