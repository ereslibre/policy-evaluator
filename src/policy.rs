use anyhow::Result;
use std::clone::Clone;

use crate::policy_metadata::Metadata;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Policy {
    pub id: String,
    pub wapc_id: u64,
    pub cluster_context: String,
}

#[cfg(test)]
impl Default for Policy {
    fn default() -> Self {
        Policy {
            id: String::default(),
            wapc_id: 1,
            cluster_context: String::new(),
        }
    }
}

impl Policy {
    pub(crate) fn new(id: String, wapc_id: u64) -> Result<Policy> {
        Ok(Policy { id, wapc_id, cluster_context: String::new() })
    }
}
