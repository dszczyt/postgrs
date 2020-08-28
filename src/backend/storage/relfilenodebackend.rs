use crate::backend::storage::relfilenode::RelFileNode;

pub struct RelFileNodeBackend {
  pub node: RelFileNode,
  pub backend_id: i32,
}
