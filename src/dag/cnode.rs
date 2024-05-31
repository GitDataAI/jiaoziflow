use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::base::{BaseNode, NodeType};

#[derive(Serialize, Deserialize, Debug)]
pub struct ComputeUnit {
    pub id: Uuid,
    pub name: String,
    pub image: String,
    pub cmd: Vec<String>,

    pub(crate) dependency: Vec<Uuid>,
    pub(crate) node_type: NodeType,
}

impl BaseNode for ComputeUnit {
    fn id(&self) -> Uuid {
        self.id
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}
