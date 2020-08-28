use crate::backend::storage::buffer::ForkNumber;
use crate::backend::storage::relfilenode::RelFileNode;
use crate::backend::storage::relfilenodebackend::RelFileNodeBackend;
use crate::types::oid::{Oid, DEFAULTTABLESPACE, GLOBALTABLESPACE};

pub fn relpathbackend(r: RelFileNode, backend_id: i32, forknum: ForkNumber) -> String {
  get_relation_path(r.db_node, r.spc_node, r.rel_node, backend_id, forknum)
}

pub fn relpath(r: RelFileNodeBackend, forknum: ForkNumber) -> String {
  relpathbackend(r.node, r.backend_id, forknum)
}

pub fn get_relation_path(
  db_node: Oid,
  spc_node: Oid,
  rel_node: Oid,
  backend_id: i32,
  forknum: ForkNumber,
) -> String {
  match spc_node {
    GLOBALTABLESPACE => match forknum {
      ForkNumber::Main => "".to_owned(),
      _ => "".to_owned(),
    },
    DEFAULTTABLESPACE => "".to_owned(),
    _ => "".to_owned(),
  }
}
